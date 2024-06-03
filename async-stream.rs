use async_stream::stream;
use futures::StreamExt;

std::include!("settings.rs");

// fn new<T: BenchType + 'static>(cap: Option<usize>) -> impl Stream<Item = T> {
//     match cap {
//       Some(0) => {
//         stream! {
//             for i in 0..MESSAGES {
//                 yield T::new(i);
//             }
//         }
//       },
//       Some(_) => unreachable!(),
//       None => unreachable!(),
        
//     }
// }

async fn spsc<T: BenchType + 'static>(_cap: Option<usize>) {
  let stream = stream! {
    for i in 1..=MESSAGES {
      yield T::new(i);
    }
  };
  
  let hrx = tokio::spawn(async move {
      tokio::pin!(stream);
      for _ in 1..=MESSAGES {
        stream.next().await.unwrap().test()
      }
  });

  hrx.await.unwrap();
}

std::include!("z_types.rs");
std::include!("z_run.rs");

#[tokio::main]
async fn main() {
    println!("async-stream");

    // run_async!("bounded0_mpmc(empty)", mpmc::<BenchEmpty>(Some(0)));
    // run_async!("bounded0_mpsc(empty)", mpsc::<BenchEmpty>(Some(0)));
    run_async!("bounded0_spsc(empty)", spsc::<BenchEmpty>(Some(0)));
    // run_async!("bounded1_mpmc(empty)", mpmc::<BenchEmpty>(Some(1)));
    // run_async!("bounded1_mpsc(empty)", mpsc::<BenchEmpty>(Some(1)));
    // run_async!("bounded1_spsc(empty)", spsc::<BenchEmpty>(Some(1)));
    // run_async!("bounded_mpmc(empty)", mpmc::<BenchEmpty>(Some(MESSAGES)));
    // run_async!("bounded_mpsc(empty)", mpsc::<BenchEmpty>(Some(MESSAGES)));
    // run_async!("bounded_seq(empty)", seq::<BenchEmpty>(Some(MESSAGES)));
    // run_async!("bounded_spsc(empty)", spsc::<BenchEmpty>(Some(MESSAGES)));
    // run_async!("unbounded_mpmc(empty)", mpmc::<BenchEmpty>(None));
    // run_async!("unbounded_mpsc(empty)", mpsc::<BenchEmpty>(None));
    // run_async!("unbounded_seq(empty)", seq::<BenchEmpty>(None));
    // run_async!("unbounded_spsc(empty)", spsc::<BenchEmpty>(None));

    // run_async!("bounded0_mpmc(usize)", mpmc::<BenchUsize>(Some(0)));
    // run_async!("bounded0_mpsc(usize)", mpsc::<BenchUsize>(Some(0)));
    run_async!("bounded0_spsc(usize)", spsc::<BenchUsize>(Some(0)));
    // run_async!("bounded1_mpmc(usize)", mpmc::<BenchUsize>(Some(1)));
    // run_async!("bounded1_mpsc(usize)", mpsc::<BenchUsize>(Some(1)));
    // run_async!("bounded1_spsc(usize)", spsc::<BenchUsize>(Some(1)));
    // run_async!("bounded_mpmc(usize)", mpmc::<BenchUsize>(Some(MESSAGES)));
    // run_async!("bounded_mpsc(usize)", mpsc::<BenchUsize>(Some(MESSAGES)));
    // run_async!("bounded_seq(usize)", seq::<BenchUsize>(Some(MESSAGES)));
    // run_async!("bounded_spsc(usize)", spsc::<BenchUsize>(Some(MESSAGES)));
    // run_async!("unbounded_mpmc(usize)", mpmc::<BenchUsize>(None));
    // run_async!("unbounded_mpsc(usize)", mpsc::<BenchUsize>(None));
    // run_async!("unbounded_seq(usize)", seq::<BenchUsize>(None));
    // run_async!("unbounded_spsc(usize)", spsc::<BenchUsize>(None));

    // run_async!("bounded0_mpmc(big)", mpmc::<BenchFixedArray>(Some(0)));
    // run_async!("bounded0_mpsc(big)", mpsc::<BenchFixedArray>(Some(0)));
    run_async!("bounded0_spsc(big)", spsc::<BenchFixedArray>(Some(0)));
    // run_async!("bounded1_mpmc(big)", mpmc::<BenchFixedArray>(Some(1)));
    // run_async!("bounded1_mpsc(big)", mpsc::<BenchFixedArray>(Some(1)));
    // run_async!("bounded1_spsc(big)", spsc::<BenchFixedArray>(Some(1)));
    // run_async!("bounded_mpmc(big)", mpmc::<BenchFixedArray>(Some(MESSAGES)));
    // run_async!("bounded_mpsc(big)", mpsc::<BenchFixedArray>(Some(MESSAGES)));
    // run_async!("bounded_seq(big)", seq::<BenchFixedArray>(Some(MESSAGES)));
    // run_async!("bounded_spsc(big)", spsc::<BenchFixedArray>(Some(MESSAGES)));
    // run_async!("unbounded_mpmc(big)", mpmc::<BenchFixedArray>(None));
    // run_async!("unbounded_mpsc(big)", mpsc::<BenchFixedArray>(None));
    // run_async!("unbounded_seq(big)", seq::<BenchFixedArray>(None));
    // run_async!("unbounded_spsc(big)", spsc::<BenchFixedArray>(None));
}
