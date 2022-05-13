use allo_isolate::Isolate;
use membrane_types::Callback;
use serde::ser::Serialize;

pub fn send_callback<T: Serialize, E: Serialize>(port: i64) -> impl Callback<Result<T, E>> {
  move |result: Result<T, E>| {
    let isolate = Isolate::new(port);
    send::<&T, &E>(isolate, result.as_ref());
  }
}

pub fn send_stream_callback<T: Serialize, E: Serialize>(port: i64) -> impl Callback<Result<T, E>> {
  move |result: Result<T, E>| {
    let isolate = Isolate::new(port);
    send::<&T, &E>(isolate, result.as_ref());
  }
}

pub fn send<T: Serialize, E: Serialize>(isolate: Isolate, result: Result<T, E>) {
  match result {
    Ok(value) => {
      if let Ok(buffer) = crate::bincode::serialize(&(true, value)) {
        isolate.post(crate::allo_isolate::ZeroCopyBuffer(buffer));
      }
    }
    Err(err) => {
      if let Ok(buffer) = crate::bincode::serialize(&(false, err)) {
        isolate.post(crate::allo_isolate::ZeroCopyBuffer(buffer));
      }
    }
  };
}
