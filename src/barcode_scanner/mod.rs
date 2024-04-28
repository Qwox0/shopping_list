mod barcode;
mod error;

use crate::{camera::CameraService, option_signal::OptionSignal};
pub use barcode::*;
pub use error::BarcodeError;
use leptos::{
    html::{Canvas, Video},
    leptos_dom::helpers::IntervalHandle,
    *,
};
use rxing::{
    common::HybridBinarizer, oned::MultiFormatUPCEANReader, BinaryBitmap, DecodingHintDictionary,
    Luma8LuminanceSource, Reader,
};
use rxing_wasm::BarcodeResult;
use std::{fmt::Debug, time::Duration};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

pub fn get_2d_context(
    canvas: &HtmlCanvasElement,
) -> Result<CanvasRenderingContext2d, BarcodeError> {
    canvas
        .get_context("2d")
        .map_err(|_| BarcodeError::GetCanvasContextErr)?
        .ok_or(BarcodeError::GetCanvasContextErr)?
        .dyn_into()
        .map_err(|_| BarcodeError::JsDynCastError)
}

#[derive(Debug, Clone, Copy)]
enum ScanCount {
    Zero,
    Count { barcode: Barcode, count: u8 },
}

impl ScanCount {
    fn update(&mut self, next: Option<Barcode>) {
        next.as_ref().inspect(|b| logging::log!("found barcode: {:?}", b));

        *self = match (*self, next) {
            (_, None) => ScanCount::Zero,
            (ScanCount::Count { barcode: kind, count }, Some(barcode)) if kind == barcode => {
                ScanCount::Count { barcode: kind, count: count + 1 }
            },
            (_, Some(barcode)) => ScanCount::Count { barcode, count: 1 },
        }
    }
}

fn scan_for_barcode(
    video: &HtmlVideoElement,
    canvas: &HtmlCanvasElement,
) -> Result<Option<Barcode>, BarcodeError> {
    let v_width_int = video.video_width();
    let v_height_int = video.video_height();
    if v_width_int == 0 || v_height_int == 0 {
        return Ok(None);
    }

    web_sys::console::time_with_label("scan_for_barcode full");
    let v_width = v_width_int as f64;
    /*
    let v_height = v_height_int as f64;
    canvas.set_width(v_width_int);
    canvas.set_height(v_height_int);
    let context = get_2d_context(&canvas)?;

    time_with_label("draw");
    context
        .draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &video, 0.0, 0.0, v_width, v_height, 0.0, 0.0, v_width, v_height,
        )
        .map_err(BarcodeError::DrawImgErr)?;
    time_end_with_label("draw");

    let img_data = context
        .get_image_data(0.0, 0.0, v_width, v_height)
        .map_err(BarcodeError::GetImgDataErr)?
        .data();
    */

    const CANVAS_HEIGHT: u32 = 100;
    canvas.set_width(v_width_int);
    canvas.set_height(CANVAS_HEIGHT);
    let dy = v_height_int.saturating_sub(CANVAS_HEIGHT).div_euclid(2) as f64;

    let context = get_2d_context(&canvas)?;

    // time_with_label("draw");
    context
        .draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &video,
            0.0,
            dy,
            v_width,
            CANVAS_HEIGHT as f64,
            0.0,
            0.0,
            v_width,
            CANVAS_HEIGHT as f64,
        )
        .map_err(BarcodeError::DrawImgErr)?;
    // time_end_with_label("draw"); // ~ 2ms

    let img_data = context
        .get_image_data(0.0, 0.0, v_width, CANVAS_HEIGHT as f64)
        .map_err(BarcodeError::GetImgDataErr)?
        .data();

    // time_with_label("luma");
    let luma_img = rxing_wasm::convert_js_image_to_luma(&img_data);
    // time_end_with_label("luma"); // ~ 10ms

    // time_with_label("decode_time");
    let luma_img = Luma8LuminanceSource::new(luma_img, v_width_int, CANVAS_HEIGHT);
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(luma_img));
    let hints = DecodingHintDictionary::new();
    let barcode = MultiFormatUPCEANReader::new(&hints).decode_with_hints(&mut bitmap, &hints);

    let barcode = barcode.map(BarcodeResult::from).ok();
    // time_end_with_label("decode_time"); // ~ 0.1ms

    web_sys::console::time_end_with_label("scan_for_barcode full"); // ~ 30ms

    barcode.map(Barcode::try_from).transpose()
}

#[component]
pub fn BarcodeScanner<F>(set_barcode: F) -> impl IntoView
where F: Fn(Barcode) + Copy + 'static {
    let barcode_scanner_error = OptionSignal::<RwSignal<Option<BarcodeError>>>::new();

    create_effect(move |_| {
        if let Some(err) = barcode_scanner_error() {
            logging::error!("ERROR (BarcodeScanner): {:#?}", err);
        }
    });

    let video = create_node_ref::<Video>();
    let canvas = create_node_ref::<Canvas>();

    #[cfg(feature = "hydrate")]
    {
        let camera = CameraService::from_context();
        let video_stream = camera.open();
        on_cleanup(move || camera.close());

        let video_stream = move || {
            video_stream
                .get()
                .flatten()
                .transpose()
                .map_err(|e| barcode_scanner_error(e.into()))
                .unwrap_or_default()
        };

        let (barcode_count, set_barcode_count) = create_signal(ScanCount::Zero);

        let scan_interval = set_interval_with_handle(
            move || {
                let Some(video) = video() else { return };
                let Some(canvas) = canvas() else { return };
                match scan_for_barcode(&video, &canvas) {
                    Ok(barcode) => set_barcode_count.update(|c| c.update(barcode)),
                    Err(err) => barcode_scanner_error(err),
                }
            },
            Duration::from_millis(50),
        );

        let scan_interval = scan_interval
            .map_err(|err| barcode_scanner_error(BarcodeError::SetIntervalErr(err)))
            .ok();
        on_cleanup(move || scan_interval.iter().for_each(IntervalHandle::clear));

        const NEEDED_SCAN_COUNT: u8 = 3;
        create_effect(move |_| match barcode_count() {
            ScanCount::Count { barcode, count } if count >= NEEDED_SCAN_COUNT => {
                set_barcode(barcode);
            },
            _ => (),
        });

        create_effect(move |_| match (video(), video_stream()) {
            (Some(video), Some(stream)) => video.set_src_object(Some(&stream)),
            //(None, Some(_)) => panic!("got the video stream before the video element"),
            _ => (),
        });
    }

    let error_text =
        move || barcode_scanner_error().as_ref().map(ToString::to_string).unwrap_or_default();
    let error_view = move || view! { <span class="error"> { error_text } </span> };

    view! {
        <Show
            when=move || barcode_scanner_error.with(Option::is_none)
            fallback=error_view
        >
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <video ref_=video id="camera-video" playsinline autoplay muted />
            </Transition>
            <canvas ref_=canvas hidden />
        </Show>
    }
}
