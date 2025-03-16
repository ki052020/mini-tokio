use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::time::{Duration, Instant};
use futures::task;

mod delay;
use delay::*;

// ----------------------------------------------------------------
fn main() {
	let mut mini_tokio = MiniTokio::new();

	mini_tokio.spawn(async {
		let when = Instant::now() + Duration::from_secs(5);
		let future = Delay { when };

		let rslt = future.await;
		println!("--- rslt -> {}", rslt);
//		assert_eq!(out, "done");
	});

	mini_tokio.run();
}

// ----------------------------------------------------------------
struct MiniTokio {
   tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
	fn new() -> MiniTokio {
		MiniTokio {
			tasks: VecDeque::new(),
		}
	}

	/// mini-tokio のインスタンスに "future" を渡す
	fn spawn<F>(&mut self, future: F)
		where
			F: Future<Output = ()> + Send + 'static,
	{
		self.tasks.push_back(Box::pin(future));
	}

	fn run(&mut self) {
		let waker = task::noop_waker();
		let mut cx = Context::from_waker(&waker);

		while let Some(mut task) = self.tasks.pop_front() {
			if task.as_mut().poll(&mut cx).is_pending() {
				self.tasks.push_back(task);
			}
		}
	}
}


// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
	#[cfg(feature = "test_1")]
	#[test]
	fn test_1() {
		let mut a: i32 = 2;  // i32 は不要
		let mut b = 200;
		println!("(a, b) = ({a:?}, {b:?})");
		println!("(pa, pb) = ({:p}, {:p})", &a, &b);
		
		core::mem::swap(&mut a, &mut b);
		println!("(a, b) = ({a:?}, {b:?})");
		println!("(pa, pb) = ({:p}, {:p})", &a, &b);
	}

	#[cfg(feature = "test_2")]
	#[test]
	fn test_2() {
		use std::any::type_name_of_val;
		
		let mut a: i32 = 2;
		println!("a -> {a:?}");

		let pa = &mut a;
		println!("*pa -> {}", *pa);
		println!("type of pa -> {}", type_name_of_val(pa));
		println!("type of &pa -> {}", type_name_of_val(&pa));
		*pa = 200;
		
		println!("a -> {a:?}");
	}

/*
pub struct std::pin::Pin<Ptr> {
    pub __pointer: Ptr,
}
*/
	#[cfg(feature = "test_3")]
	#[test]
	fn test_3() {
		use std::any::type_name_of_val;
		use std::pin::Pin;
		use std::borrow::BorrowMut;
		
		let a = 2;
		let b = 200;
		let mut c = Pin::new(&a);
		let mut d = Pin::new(&b);
				
		println!("type of c -> {}", type_name_of_val(&c));
		
		println!("\n(a, b) -> ({a:?}, {b:?}) / ({:p}, {:p})", &a, &b);
		println!("(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
		println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
		
		core::mem::swap(c.borrow_mut(), d.borrow_mut());
		println!("\n(a, b) -> ({a:?}, {b:?}) / ({:p}, {:p})", &a, &b);
		println!("(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
		println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
	}

	#[cfg(feature = "test_4")]
	#[test]
	fn test_4() {
		use std::any::type_name_of_val;
		use std::pin::Pin;
		use std::borrow::BorrowMut;
		
		let mut a = 2;
		let mut b = 200;
		println!("(a, b) -> ({a:?}, {b:?}) / ({:p}, {:p})", &a, &b);
		{
			let mut c = Pin::new(&mut a);
			let mut d = Pin::new(&mut b);
					
			println!("\ntype of c -> {}", type_name_of_val(&c));
			
//			println!("\n(a, b) -> ({a:?}, {b:?}) / ({:p}, {:p})", &a, &b);  // borrow error
			println!("\n(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
			println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
			
			core::mem::swap::<i32>(c.borrow_mut(), d.borrow_mut());
			println!("(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
			println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
		}
		println!("\n(a, b) -> ({a:?}, {b:?}) / ({:p}, {:p})", &a, &b);
	}
	
	#[cfg(feature = "test_5")]
	#[test]
	fn test_5() {
		use std::any::type_name_of_val;
		use std::pin::pin;
		use std::borrow::BorrowMut;
		use std::marker::PhantomPinned;
	
		#[derive(Debug)]
		struct S {
			_x: i32,
			_y: PhantomPinned,
		}
		
		impl S {
			fn new(x: i32) -> S {
				S{ _x: x , _y: PhantomPinned::default() }
			}
		}
		
		let a = S::new(2);
		let b = S::new(200);

		println!("type of a -> {}", type_name_of_val(&a));
		println!("a -> {a:?}");
		
		// 以下の行はエラーとなる
		// 理由：PhantomPinned が Unpin を実装していないため
//		let mut c = Pin::new(&a);

		let mut c = pin!(&a);
		let mut d = pin!(&b);
		
		// type of c -> Pin<&mut &S>
		// C で言うと、c ＝「S* const* をメンバ変数に持つもの」
		println!("\ntype of c -> {}", type_name_of_val(&c));
		println!("\n(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
		println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
		
		// c と d のフィールド（S const* へのポインタ）は書きかえることができる
		core::mem::swap(c.borrow_mut(), d.borrow_mut());
		println!("\n(c, d) -> ({c:?}, {d:?}) / ({c:p}, {d:p})");
		println!("(&c, &d) -> ({0:?}, {1:?}) / ({0:p}, {1:p})", &c, &d);
	}
	
}
