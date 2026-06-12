use core::fmt::Debug;

pub enum LfsError<E: Debug + Copy + PartialEq> {
    Io(E),       // Error during device operation
    Corrupt,     // Corrupted
    NoEntry,     // No directory entry
    Exist,       // Entry already exists
    NotDir,      // Entry is not a dir
    IsDir,       // Entry is a dir
    NotEmpty,    // Dir is not empty
    BadFileNb,   // Bad file number
    FileTooBig,  // File too large
    Invalid,     // Invalid parameter
    NoSpace,     // No space left on device
    NoMem,       // No more memory available
    NoAttr,      // No data/attr available
    NameTooLong, // File name too long
}

pub type LfsResult<T, E> = Result<T, LfsError<E>>;
