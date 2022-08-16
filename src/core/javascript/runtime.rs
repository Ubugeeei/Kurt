use rusty_v8 as v8;
use std::{cell::RefCell, rc::Rc, sync::Once};

pub struct JavaScriptRuntimeState {
  pub context: v8::Global<v8::Context>,
}

#[derive(Debug)]
pub struct JavaScriptRuntime {
  v8_isolate: v8::OwnedIsolate,
}

impl JavaScriptRuntime {
  /// Singleton
  pub fn new() -> Self {
    // initialize V8
    static INIT_V8: Once = Once::new();
    INIT_V8.call_once(move || {
      let platform = v8::new_default_platform().unwrap();
      v8::V8::initialize_platform(platform);
      v8::V8::initialize();
    });

    // create v8 isolate & context
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let context = {
      let isolate_scope = &mut v8::HandleScope::new(&mut isolate);
      let handle_scope = &mut v8::EscapableHandleScope::new(isolate_scope);
      let context = v8::Context::new(handle_scope);
      let context_scope = handle_scope.escape(context);
      v8::Global::new(handle_scope, context_scope)
    };

    // store state inside v8 isolate
    isolate.set_slot(Rc::new(RefCell::new(JavaScriptRuntimeState { context })));

    JavaScriptRuntime {
      v8_isolate: isolate,
    }
  }

  pub fn execute(&mut self, filename: &str, source: &str) -> Result<String, String> {
    let scope = &mut self.get_handle_scope();

    let source = v8::String::new(scope, source).unwrap();
    let source_map = v8::undefined(scope);
    let name = v8::String::new(scope, filename).unwrap();
    let origin = v8::ScriptOrigin::new(
      scope,
      name.into(),
      0,
      0,
      false,
      0,
      source_map.into(),
      false,
      false,
      false,
    );

    let mut tc_scope = v8::TryCatch::new(scope);
    let script = match v8::Script::compile(&mut tc_scope, source, Some(&origin)) {
      Some(script) => script,
      None => {
        assert!(tc_scope.has_caught());
        return Err(to_pretty_string(tc_scope));
      }
    };

    match script.run(&mut tc_scope) {
      Some(result) => Ok(
        result
          .to_string(&mut tc_scope)
          .unwrap()
          .to_rust_string_lossy(&mut tc_scope),
      ),
      None => {
        assert!(tc_scope.has_caught());
        Err(to_pretty_string(tc_scope))
      }
    }
  }
}

impl JavaScriptRuntime {
  pub fn get_handle_scope(&mut self) -> v8::HandleScope {
    let context = self.get_context();
    v8::HandleScope::with_context(&mut self.v8_isolate, context)
  }

  pub fn get_context(&mut self) -> v8::Global<v8::Context> {
    let state = self.get_state();
    let state = state.borrow();
    state.context.clone()
  }

  pub fn get_state(&self) -> Rc<RefCell<JavaScriptRuntimeState>> {
    Self::state(&self.v8_isolate)
  }

  pub fn state(isolate: &v8::Isolate) -> Rc<RefCell<JavaScriptRuntimeState>> {
    let s = isolate
      .get_slot::<Rc<RefCell<JavaScriptRuntimeState>>>()
      .unwrap();
    s.clone()
  }
}

fn to_pretty_string(mut try_catch: v8::TryCatch<v8::HandleScope>) -> String {
  // TODO (enhancement): better error handling needed! wanna remove uncareful unwrap().
  let exception_string = try_catch
    .exception()
    .unwrap()
    .to_string(&mut try_catch)
    .unwrap()
    .to_rust_string_lossy(&mut try_catch);
  let message = try_catch.message().unwrap();

  let filename = message
    .get_script_resource_name(&mut try_catch)
    .map_or_else(
      || "(unknown)".into(),
      |s| {
        s.to_string(&mut try_catch)
          .unwrap()
          .to_rust_string_lossy(&mut try_catch)
      },
    );
  let line_number = message.get_line_number(&mut try_catch).unwrap_or_default();
  format!("{}:{}: {}", filename, line_number, exception_string)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_execute() {
    let mut runtime = JavaScriptRuntime::new();

    // number ops
    {
      let r = runtime.execute("", "42 * 2");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "84");
    }

    // vars
    {
      let r = runtime.execute("", "x");
      assert!(r.is_err());
    }
    {
      let r = runtime.execute("", "const x = 1; x");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "1");
    }

    // function
    {
      let r = runtime.execute("", "let double = (i) => { return i * 2 }; double(1)");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "2");
    }
    {
      let r = runtime.execute("", "double(4)");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "8");
    }
    {
      let r = runtime.execute("", "let half = (i) => i / 2; half(2)");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "1");
    }
    {
      let r = runtime.execute("", "half(4)");
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), "2");
    }
  }
}
