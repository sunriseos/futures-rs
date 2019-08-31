//! Abstractions for asynchronous programming.
//!
//! This crate provides a number of core abstractions for writing asynchronous
//! code:
//!
//! - [Futures](crate::future::Future) are single eventual values produced by
//!   asychronous computations. Some programming languages (e.g. JavaScript)
//!   call this concept "promise".
//! - [Streams](crate::stream::Stream) represent a series of values
//!   produced asynchronously.
//! - [Sinks](crate::sink::Sink) provide support for asynchronous writing of
//!   data.
//! - [Executors](crate::executor) are responsible for running asynchronous
//!   tasks.
//!
//! The crate also contains abstractions for [asynchronous I/O](crate::io) and
//! [cross-task communication](crate::channel).
//!
//! Underlying all of this is the *task system*, which is a form of lightweight
//! threading. Large asynchronous computations are built up using futures,
//! streams and sinks, and then spawned as independent tasks that are run to
//! completion, but *do not block* the thread running them.

#![cfg_attr(feature = "cfg-target-has-atomic", feature(cfg_target_has_atomic))]
#![cfg_attr(feature = "never-type", feature(never_type))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::all)]
#![doc(
    html_root_url = "https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures"
)]

#[cfg(all(feature = "async-await", not(feature = "nightly")))]
compile_error!("The `async-await` feature requires the `nightly` feature as an explicit opt-in to unstable features");

#[cfg(all(feature = "cfg-target-has-atomic", not(feature = "nightly")))]
compile_error!("The `cfg-target-has-atomic` feature requires the `nightly` feature as an explicit opt-in to unstable features");

#[cfg(all(feature = "never-type", not(feature = "nightly")))]
compile_error!("The `never-type` feature requires the `nightly` feature as an explicit opt-in to unstable features");

#[doc(hidden)]
pub use futures_util::core_reexport;

#[doc(hidden)]
pub use futures_core::future::Future;
#[doc(hidden)]
pub use futures_core::future::TryFuture;
#[doc(hidden)]
pub use futures_util::future::FutureExt;
#[doc(hidden)]
pub use futures_util::try_future::TryFutureExt;

#[doc(hidden)]
pub use futures_core::stream::Stream;
#[doc(hidden)]
pub use futures_core::stream::TryStream;
#[doc(hidden)]
pub use futures_util::stream::StreamExt;
#[doc(hidden)]
pub use futures_util::try_stream::TryStreamExt;

#[doc(hidden)]
pub use futures_sink::Sink;
#[doc(hidden)]
pub use futures_util::sink::SinkExt;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};
#[cfg(feature = "std")]
#[doc(hidden)]
pub use futures_util::{
    AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt,
};

#[doc(hidden)]
pub use futures_core::task::Poll;

// Macro reexports
#[cfg(feature = "async-await")]
pub use futures_util::{
    // Async-await
    join,
    pending,
    poll,
    try_join,
};
pub use futures_util::{
    ready,
    // Error/readiness propagation
    try_ready,
};

#[cfg(feature = "std")]
pub mod channel {
    //! Cross-task communication.
    //!
    //! Like threads, concurrent tasks sometimes need to communicate with each
    //! other. This module contains two basic abstractions for doing so:
    //!
    //! - [oneshot](crate::channel::oneshot), a way of sending a single value
    //!   from one task to another.
    //! - [mpsc](crate::channel::mpsc), a multi-producer, single-consumer
    //!   channel for sending values between tasks, analogous to the
    //!   similarly-named structure in the standard library.

    pub use futures_channel::{mpsc, oneshot};
}

#[cfg(feature = "compat")]
pub mod compat {
    //! Interop between `futures` 0.1 and 0.3.

    pub use futures_util::compat::{
        Compat, Compat01As03, Compat01As03Sink, CompatSink, Executor01As03,
        Executor01CompatExt, Executor01Future, Future01CompatExt,
        Sink01CompatExt, Stream01CompatExt,
    };

    #[cfg(feature = "io-compat")]
    pub use futures_util::compat::{
        AsyncRead01CompatExt, AsyncWrite01CompatExt,
    };
}

#[cfg(feature = "std")]
pub mod executor {
    //! Task execution.
    //!
    //! All asynchronous computation occurs within an executor, which is
    //! capable of spawning futures as tasks. This module provides several
    //! built-in executors, as well as tools for building your own.
    //!
    //! # Using a thread pool (M:N task scheduling)
    //!
    //! Most of the time tasks should be executed on a [thread
    //! pool](crate::executor::ThreadPool). A small set of worker threads can
    //! handle a very large set of spawned tasks (which are much lighter weight
    //! than threads).
    //!
    //! The simplest way to use a thread pool is to
    //! [`run`](crate::executor::ThreadPool::run) an initial task on it, which
    //! can then spawn further tasks back onto the pool to complete its work:
    //!
    //! ```
    //! use futures::executor::ThreadPool;
    //! # use futures::future::{Future, lazy};
    //! # let my_app = lazy(|_| 42);
    //!
    //! // assuming `my_app: Future`
    //! ThreadPool::new().expect("Failed to create threadpool").run(my_app);
    //! ```
    //!
    //! The call to [`run`](crate::executor::ThreadPool::run) will block the
    //! current thread until the future defined by `my_app` completes, and will
    //! return the result of that future.
    //!
    //! # Spawning additional tasks
    //!
    //! Tasks can be spawned onto a spawner by calling its
    //! [`spawn_obj`](crate::task::Spawn::spawn_obj) method directly.
    //! In the case of `!Send` futures,
    //! [`spawn_local_obj`](crate::task::LocalSpawn::spawn_local_obj)
    //! can be used instead.
    //!
    //! # Single-threaded execution
    //!
    //! In addition to thread pools, it's possible to run a task (and the tasks
    //! it spawns) entirely within a single thread via the
    //! [`LocalPool`](crate::executor::LocalPool) executor. Aside from cutting
    //! down on synchronization costs, this executor also makes it possible to
    //! spawn non-`Send` tasks, via
    //! [`spawn_local_obj`](crate::executor::LocalSpawn::spawn_local_obj).
    //! The `LocalPool` is best suited for running I/O-bound tasks that do
    //! relatively little work between I/O operations.
    //!
    //! There is also a convenience function,
    //! [`block_on`](crate::executor::block_on), for simply running a future to
    //! completion on the current thread, while routing any spawned tasks
    //! to a global thread pool.

    pub use futures_executor::{
        block_on, block_on_stream, enter, BlockingStream, Enter, EnterError,
        LocalPool, LocalSpawner, ThreadPool, ThreadPoolBuilder,
    };
}

pub mod future {
    //! Asynchronous values.
    //!
    //! This module contains:
    //!
    //! - The [`Future` trait](crate::future::Future).
    //! - The [`FutureExt`](crate::future::FutureExt) trait, which provides
    //!   adapters for chaining and composing futures.
    //! - Top-level future combinators like [`lazy`](crate::future::lazy) which
    //!   creates a future from a closure that defines its return value, and
    //!   [`ready`](crate::future::ready), which constructs a future with an
    //!   immediate defined value.

    pub use futures_core::future::{
        FusedFuture, Future, FutureObj, LocalFutureObj, TryFuture,
        UnsafeFutureObj,
    };

    #[cfg(feature = "alloc_feature")]
    pub use futures_core::future::BoxFuture;

    pub use futures_util::future::{
        empty, err, join, join3, join4, join5, lazy, maybe_done, ok, poll_fn,
        ready, select, Either, Empty, Flatten, FlattenStream, Fuse, FutureExt,
        Inspect, IntoStream, Join, Join3, Join4, Join5, Lazy, Map, MaybeDone,
        OptionFuture, PollFn, Ready, Select, Then, UnitError,
    };

    #[cfg(feature = "alloc_feature")]
    pub use futures_util::future::{join_all, select_all, JoinAll, SelectAll};

    #[cfg_attr(
        feature = "cfg-target-has-atomic",
        cfg(all(target_has_atomic = "cas", target_has_atomic = "ptr"))
    )]
    #[cfg(feature = "alloc_feature")]
    pub use futures_util::future::{
        abortable, AbortHandle, AbortRegistration, Abortable, Aborted,
    };

    #[cfg(feature = "std")]
    pub use futures_util::future::{
        // For FutureExt:
        CatchUnwind,
        Remote,
        RemoteHandle,
        Shared,
    };

    pub use futures_util::try_future::{
        try_join, try_join3, try_join4, try_join5, AndThen, ErrInto,
        FlattenSink, IntoFuture, MapErr, MapOk, OrElse, TryFutureExt, TryJoin,
        TryJoin3, TryJoin4, TryJoin5, UnwrapOrElse,
    };

    #[cfg(feature = "never-type")]
    pub use futures_util::future::NeverError;

    #[cfg(feature = "alloc_feature")]
    pub use futures_util::try_future::{
        select_ok, try_join_all, SelectOk, TryJoinAll,
    };
}

#[cfg(feature = "std")]
pub mod io {
    //! Asynchronous I/O.
    //!
    //! This module is the asynchronous version of `std::io`. It defines two
    //! traits, [`AsyncRead`](crate::io::AsyncRead) and
    //! [`AsyncWrite`](crate::io::AsyncWrite), which mirror the `Read` and
    //! `Write` traits of the standard library. However, these traits integrate
    //! with the asynchronous task system, so that if an I/O object isn't ready
    //! for reading (or writing), the thread is not blocked, and instead the
    //! current task is queued to be woken when I/O is ready.
    //!
    //! In addition, the [`AsyncReadExt`](crate::io::AsyncReadExt) and
    //! [`AsyncWriteExt`](crate::io::AsyncWriteExt) extension traits offer a
    //! variety of useful combinators for operating with asynchronous I/O
    //! objects, including ways to work with them using futures, streams and
    //! sinks.

    pub use futures_io::{
        AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, Error, ErrorKind,
        Initializer, IoSlice, IoSliceMut, Result, SeekFrom,
    };

    pub use futures_util::io::{
        AllowStdIo, AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt,
        BufReader, Close, CopyInto, Flush, Lines, Read, ReadExact, ReadHalf,
        ReadLine, ReadToEnd, ReadUntil, Seek, Window, WriteAll, WriteHalf,
    };
}

#[cfg(feature = "std")]
pub mod lock {
    //! Futures-powered synchronization primitives.
    pub use futures_util::lock::{Mutex, MutexGuard, MutexLockFuture};
}

pub mod prelude {
    //! A "prelude" for crates using the `futures` crate.
    //!
    //! This prelude is similar to the standard library's prelude in that you'll
    //! almost always want to import its entire contents, but unlike the
    //! standard library's prelude you'll have to do so manually:
    //!
    //! ```
    //! use futures::prelude::*;
    //! ```
    //!
    //! The prelude may grow over time as additional items see ubiquitous use.

    pub use crate::future::{self, Future, FutureExt, TryFuture, TryFutureExt};
    pub use crate::sink::{self, Sink, SinkExt};
    pub use crate::stream::{self, Stream, StreamExt, TryStream, TryStreamExt};

    #[cfg(feature = "std")]
    pub use crate::io::{
        AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncSeek,
        AsyncSeekExt, AsyncWrite, AsyncWriteExt,
    };
}

pub mod sink {
    //! Asynchronous sinks.
    //!
    //! This module contains:
    //!
    //! - The [`Sink` trait](crate::sink::Sink), which allows you to
    //!   asynchronously write data.
    //! - The [`SinkExt`](crate::sink::SinkExt) trait, which provides adapters
    //!   for chaining and composing sinks.

    pub use futures_sink::Sink;

    pub use futures_util::sink::{
        drain, Close, Drain, DrainError, Fanout, Flush, Send, SendAll,
        SinkErrInto, SinkExt, SinkMapErr, With, WithFlatMap,
    };

    #[cfg(feature = "alloc_feature")]
    pub use futures_util::sink::Buffer;
}

pub mod stream {
    //! Asynchronous streams.
    //!
    //! This module contains:
    //!
    //! - The [`Stream` trait](crate::stream::Stream), for objects that can
    //!   asynchronously produce a sequence of values.
    //! - The [`StreamExt`](crate::stream::StreamExt) trait, which provides
    //!   adapters for chaining and composing streams.
    //! - Top-level stream contructors like [`iter`](crate::stream::iter)
    //!   which creates a stream from an iterator.

    pub use futures_core::stream::{FusedStream, Stream, TryStream};

    #[cfg(feature = "alloc_feature")]
    pub use futures_core::stream::BoxStream;

    pub use futures_util::stream::{
        empty, iter, once, poll_fn, repeat, select, unfold, Chain, Collect,
        Concat, Empty, Enumerate, Filter, FilterMap, Flatten, Fold, ForEach,
        Forward, Fuse, Inspect, Iter, Map, Next, Once, Peekable, PollFn,
        Repeat, Select, SelectNextSome, Skip, SkipWhile, StreamExt,
        StreamFuture, Take, TakeWhile, Then, Unfold, Zip,
    };

    #[cfg(feature = "alloc_feature")]
    pub use futures_util::stream::Chunks;

    #[cfg_attr(
        feature = "cfg-target-has-atomic",
        cfg(all(target_has_atomic = "cas", target_has_atomic = "ptr"))
    )]
    #[cfg(feature = "alloc_feature")]
    pub use futures_util::stream::{
        futures_unordered,
        select_all,
        // For StreamExt:
        BufferUnordered,
        Buffered,
        ForEachConcurrent,
        FuturesOrdered,
        FuturesUnordered,

        ReuniteError,

        SelectAll,
        SplitSink,
        SplitStream,
    };

    #[cfg(feature = "std")]
    pub use futures_util::stream::CatchUnwind;

    pub use futures_util::try_stream::{
        AndThen, ErrInto, InspectErr, InspectOk, IntoStream, MapErr, MapOk,
        OrElse, TryCollect, TryFilterMap, TryFold, TryForEach, TryNext,
        TrySkipWhile, TryStreamExt,
    };

    #[cfg_attr(
        feature = "cfg-target-has-atomic",
        cfg(all(target_has_atomic = "cas", target_has_atomic = "ptr"))
    )]
    #[cfg(feature = "alloc_feature")]
    pub use futures_util::try_stream::{
        // For TryStreamExt:
        TryBufferUnordered,
        TryForEachConcurrent,
    };

    #[cfg(feature = "std")]
    pub use futures_util::try_stream::IntoAsyncRead;
}

pub mod task {
    //! Tools for working with tasks.
    //!
    //! This module contains:
    //!
    //! - [`Spawn`](crate::task::Spawn), a trait for spawning new tasks.
    //! - [`Context`](crate::task::Context), a context of an asynchronous task,
    //!   including a handle for waking up the task.
    //! - [`Waker`](crate::task::Waker), a handle for waking up a task.
    //!
    //! The remaining types and traits in the module are used for implementing
    //! executors or dealing with synchronization issues around task wakeup.

    pub use futures_core::task::{
        Context, LocalSpawn, Poll, RawWaker, RawWakerVTable, Spawn, SpawnError,
        Waker,
    };

    pub use futures_util::task::noop_waker;

    #[cfg(feature = "std")]
    pub use futures_util::task::noop_waker_ref;

    #[cfg(feature = "alloc_feature")]
    pub use futures_util::task::{LocalSpawnExt, SpawnExt};

    #[cfg_attr(
        feature = "cfg-target-has-atomic",
        cfg(all(target_has_atomic = "cas", target_has_atomic = "ptr"))
    )]
    #[cfg(feature = "alloc_feature")]
    pub use futures_util::task::{waker_ref, ArcWake, WakerRef};

    #[cfg_attr(
        feature = "cfg-target-has-atomic",
        cfg(all(target_has_atomic = "cas", target_has_atomic = "ptr"))
    )]
    pub use futures_util::task::AtomicWaker;
}

// `select!` re-export --------------------------------------

#[cfg(feature = "async-await")]
#[doc(hidden)]
pub use futures_util::rand_reexport;

#[cfg(feature = "async-await")]
#[doc(hidden)]
pub mod inner_select {
    pub use futures_util::select;
}

#[cfg(feature = "async-await")]
futures_util::document_select_macro! {
    #[macro_export]
    macro_rules! select { // replace `::futures_util` with `::futures` as the crate path
        ($($tokens:tt)*) => {
            $crate::inner_select::select! {
                futures_crate_path ( ::futures )
                $( $tokens )*
            }
        }
    }
}
