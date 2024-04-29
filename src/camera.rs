use crate::util::{force_use_context, IntoJsFuture, JsSet, JsSetError};
use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys::Object, MediaStream, MediaStreamConstraints, MediaStreamTrack};

#[derive(Debug, Clone, Copy)]
pub struct CameraService {
    stream_resource: Resource<bool, Option<Result<MediaStream, CameraError>>>,
    opened: RwSignal<bool>,
}

async fn get_camera_stream() -> Result<MediaStream, CameraError> {
    let mut video = Object::new();
    video.set("facingMode", &"environment".into())?;

    window()
        .navigator()
        .media_devices()
        .map_err(CameraError::NoMediaDevices)?
        .get_user_media_with_constraints(
            MediaStreamConstraints::new().audio(&JsValue::FALSE).video(&video),
        )
        .map_err(CameraError::GetUserMediaErr)?
        .into_future()
        .await
        .map_err(CameraError::GetUserMediaRejected)?
        .dyn_into()
        .map_err(CameraError::MediaStreamConversion)
}

fn close_stream(stream: &MediaStream) {
    logging::log!("cleanup MediaStream");
    stream
        .get_tracks()
        .for_each(&mut |track, _, _| match track.dyn_into::<MediaStreamTrack>() {
            Ok(track) => track.stop(),
            Err(e) => logging::error!(
                "Error while closing MediaStream: coundn't convert to MediaStreamTrack type: {:?}",
                e
            ),
        })
}

impl CameraService {
    pub fn new() -> CameraService {
        let opened = create_rw_signal(false);
        let stream_resource = create_local_resource(
            move || opened(),
            move |open| async move {
                logging::log!("camera open? {}", open);
                if open { Some(get_camera_stream().await.map_err(Into::into)) } else { None }
            },
        );
        create_effect(move |_| {
            stream_resource.with(move |stream_resource| match (opened(), stream_resource) {
                (false, Some(Some(Ok(stream)))) => close_stream(stream),
                _ => (),
            });
        });
        CameraService { stream_resource, opened }
    }

    /// # Panics
    ///
    /// Panics if CameraService doesn't exist
    pub fn from_context() -> CameraService {
        force_use_context()
    }

    pub fn open(&self) -> Resource<bool, Option<Result<MediaStream, CameraError>>> {
        if !self.opened.get_untracked() {
            self.opened.set(true);
        }
        self.stream_resource
    }

    pub fn close(&self) {
        self.opened.set(false);
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CameraError {
    #[error("couldn't get media device: {:?}", .0)]
    NoMediaDevices(JsValue),

    #[error("error while getting user media: {:?}", .0)]
    GetUserMediaErr(JsValue),

    #[error("get_user_media promise was rejected: {:?}", .0)]
    GetUserMediaRejected(JsValue),

    #[error("couldn't convert JsValue to MediaStream: {:?}", .0)]
    MediaStreamConversion(JsValue),

    #[error(transparent)]
    JsSetError(#[from] JsSetError),
}
