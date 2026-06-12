use core::fmt::Debug;

use crate::error::LfsResult;
pub use crate::types::*;

/// Block device interface
pub trait BlockDevice {
    type Error: Debug + Copy + PartialEq;

    const READ_SIZE: Size;
    const PROG_SIZE: Size;
    const BLOCK_SIZE: Size;

    /// Read a region in a block.
    ///
    /// # Invariants (guaranteed by the filesystem, not the caller)
    /// - `block < Lfs::block_count`
    /// - `off % Self::READ_SIZE == 0`
    /// - `buf.len() % Self::READ_SIZE == 0`
    /// - `buf.len() == Lfs::CACHE_SIZE`
    /// - `off + buf.len() <= Self::BLOCK_SIZE`
    /// # Errors
    /// Implementation-defined
    fn read(&self, block: BlockIdx, off: Offset, buf: &mut [u8]) -> LfsResult<(), Self::Error>;

    /// Program a region in a block. The block must have previously been erased.
    ///
    /// # Invariants (guaranteed by the filesystem, not the caller)
    /// - `block < Lfs::block_count`
    /// - `off % Self::PROG_SIZE == 0`
    /// - `data.len() % Self::PROG_SIZE == 0`
    /// - `data.len() == Lfs::CACHE_SIZE`
    /// - `off + data.len() <= Self::BLOCK_SIZE`
    /// - [`erase`](BlockDevice::erase) has been called on this block before its first `prog`
    /// - Each `CACHE_SIZE`-aligned region of a block is programmed at most once per erase
    ///   cycle — the prog cache merges partial writes internally before calling `prog`, so
    ///   the hardware callback never sees overlapping ranges
    /// # Errors
    /// - May return [`LfsError::Corrupt`] if the block should be considered bad
    fn prog(&mut self, block: BlockIdx, off: Offset, data: &[u8]) -> LfsResult<(), Self::Error>;

    /// Erase a block. A block must be erased before being programmed.
    /// The state of an erased block is undefined.
    ///
    /// # Invariants (guaranteed by the filesystem, not the caller)
    /// - `block < Lfs::block_count`
    /// - The next `prog` on this block will always start at `off = 0`
    /// # Errors
    /// - May return [`LfsError::Corrupt`] if the block should be considered bad
    fn erase(&mut self, block: BlockIdx) -> LfsResult<(), Self::Error>;

    /// Sync the state of the underlying block device.
    ///
    /// # Postcondition
    /// - All prior [`prog`](BlockDevice::prog) operations are durable and visible to
    ///   subsequent [`read`](BlockDevice::read) calls
    /// # Errors
    /// Implementation-defined
    fn sync(&mut self) -> LfsResult<(), Self::Error>;

    /// Lock the underlying block device.
    /// # Errors
    /// Implementation-defined
    #[cfg(feature = "threadsafe")]
    fn lock(&mut self) -> LfsResult<(), Self::Error>;

    /// Unlock the underlying block device.
    /// # Errors
    /// Implementation-defined
    #[cfg(feature = "threadsafe")]
    fn unlock(&mut self) -> LfsResult<(), Self::Error>;
}
