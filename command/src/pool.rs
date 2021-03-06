//! Pool module docs.

use std::fmt::Debug;

use relevant::Relevant;

use buffer::*;
use capability::*;
use device::{CommandBuffer, Device};
use family::FamilyId;
use frame::{CompleteFrame, Frame, FrameBound, FrameIndex};

/// Simple pool wrapper.
/// Doesn't provide any guarantees.
/// Wraps raw buffers into `Buffer`.
#[derive(Debug)]
pub struct Pool<P, C, R = ()> {
    inner: P,
    capability: C,
    reset: R,
    family: FamilyId,
    relevant: Relevant,
}

impl<P, C, R> Pool<P, C, R> {
    /// Allocate new buffer.
    fn allocate_buffers<D, L>(
        &mut self,
        device: &D,
        level: L,
        count: usize,
    ) -> Vec<Buffer<D::CommandBuffer, C, InitialState, L, R>>
    where
        P: Debug,
        D: Device<CommandPool = P>,
    {
        unimplemented!()
    }

    /// Free buffers.
    /// Buffers must be in droppable state.
    fn free_buffers<D, L, S>(
        &mut self,
        device: &D,
        buffers: Vec<Buffer<D::CommandBuffer, C, S, L, R>>,
    ) where
        P: Debug,
        D: Device<CommandPool = P>,
        S: Droppable,
    {
        unimplemented!()
    }

    /// Reset all buffers of this pool.
    pub unsafe fn reset(&mut self) {
        unimplemented!()
    }
}

impl<P, R> Pool<P, CapabilityFlags, R> {
    /// Convert capability level
    pub fn cast_capability<C>(self) -> Result<Pool<P, C, R>, Self>
    where
        C: Capability,
    {
        if let Some(capability) = C::from_flags(self.capability) {
            Ok(Pool {
                inner: self.inner,
                capability,
                reset: self.reset,
                family: self.family,
                relevant: self.relevant,
            })
        } else {
            Err(self)
        }
    }
}

/// Command pool that owns allocated buffers.
/// It can be used to borrow buffers one by one.
/// All buffers will be reset together via pool.
/// Prior reset user must ensure all buffers are complete.
#[derive(Debug)]
pub struct OwningPool<P, B, C, R = ()> {
    inner: Pool<P, C, R>,
    buffers: Vec<B>,
    next: usize,
}

impl<P, B, C, R> OwningPool<P, B, C, R> {
    /// Reserve at least `count` buffers.
    /// Allocate if there are not enough unused buffers.
    pub fn reserve(&mut self, count: usize) {
        unimplemented!()
    }

    /// Acquire command buffer from pool.
    /// The command buffer could be submitted only as part of submission for associated frame.
    /// TODO: Check that buffer cannot be moved out.
    pub fn acquire_buffer<D, L>(
        &mut self,
        device: &D,
        level: L,
    ) -> Buffer<&mut B, C, InitialState, L>
    where
        B: CommandBuffer + Debug + 'static,
        D: Device<CommandBuffer = B, Submit = B::Submit>,
    {
        unimplemented!()
    }

    /// Reset all buffers at once.
    ///
    /// # Safety
    ///
    /// All buffers from this pool must be in resettable state.
    /// Any primary buffer that references secondary buffer from this pool will be invalidated.
    pub unsafe fn reset(&mut self) {
        unimplemented!()
    }
}

impl<P, B, R> OwningPool<P, B, CapabilityFlags, R> {
    /// Convert capability level
    pub fn cast_capability<C>(self) -> Result<OwningPool<P, B, C, R>, Self>
    where
        C: Capability,
    {
        match self.inner.cast_capability::<C>() {
            Ok(inner) => Ok(OwningPool {
                inner,
                buffers: self.buffers,
                next: self.next,
            }),
            Err(inner) => Err(OwningPool {
                inner,
                buffers: self.buffers,
                next: self.next,
            }),
        }
    }
}

/// `OwningPool` that can be bound to frame execution.
/// All command buffers acquired from bound `FramePool` are guarantee
/// to complete when frame's fence is set, and buffers can be reset.
#[derive(Debug)]
pub struct FramePool<P, B, C> {
    inner: OwningPool<P, B, C>,
    frame: Option<FrameIndex>,
}

impl<P, B, C> FramePool<P, B, C> {
    /// Bind pool to particular frame.
    ///
    /// Command pools acquired from the bound pool could be submitted only within frame borrowing lifetime.
    /// This ensures that frame's fences will be signaled after all commands from all command buffers from this pool
    /// are complete.
    ///
    /// `reset` method must be called with `CompleteFrame` created from the bound `Frame` before binding to the another `Frame`.
    ///
    /// # Panics
    ///
    /// This function will panic if pool is still bound to frame.
    ///
    pub fn bind<'a, 'b, F>(&'a mut self, frame: &'b Frame<F>) -> FrameBound<'b, F, &'a mut Self> {
        assert!(
            self.frame.is_none(),
            "`FramePool::reset` must be called before binding to another frame"
        );

        self.frame = Some(frame.index());

        FrameBound::bind(self, frame)
    }

    /// Reset all buffers at once.
    ///
    /// # Panics
    ///
    /// This function will panic if pool wasn't bound to the specified frame.
    ///
    pub fn reset<F>(&mut self, complete: &CompleteFrame<F>) {
        assert_eq!(
            self.frame.take(),
            Some(complete.index()),
            "Pool must be bound to the specified frame"
        );
        unimplemented!()
    }
}

impl<P, B> FramePool<P, B, CapabilityFlags> {
    /// Convert capability level
    pub fn cast_capability<C>(self) -> Result<FramePool<P, B, C>, Self>
    where
        C: Capability,
    {
        match self.inner.cast_capability::<C>() {
            Ok(inner) => Ok(FramePool {
                inner,
                frame: self.frame,
            }),
            Err(inner) => Err(FramePool {
                inner,
                frame: self.frame,
            }),
        }
    }
}

impl<'a, 'b, P: 'b, B: 'b, C: 'b, F: 'a> FrameBound<'a, F, &'b mut FramePool<P, B, C>> {
    /// Reserve at least `count` buffers.
    /// Allocate if there are not enough unused buffers.
    pub fn reserve(&mut self, count: usize) {
        unimplemented!()
    }

    /// Acquire command buffer from pool.
    /// The command buffer could be submitted only as part of submission for associated frame.
    /// TODO: Check that buffer cannot be moved out.
    pub fn acquire_buffer<D, L>(
        &mut self,
        device: &D,
        level: L,
    ) -> Buffer<FrameBound<'b, &mut B, F>, C, InitialState, L>
    where
        B: CommandBuffer + Debug + 'static,
        D: Device<CommandBuffer = B, Submit = B::Submit>,
    {
        unimplemented!()
    }
}
