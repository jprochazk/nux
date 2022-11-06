// //! ### Memory allocator
// //!
// //! Various buffers in the library are allocated using a special memory
// //! allocator. This allocator maintains large region of memory, which it divides
// //! into blocks of configurable size.
// //!
// //! The start of the memory region contains a `refmap`, which is used to keep
// //! track of how many references to each block exist. The allocator hands out
// //! slices of this memory region upon request. A slice may not be larger than a
// //! single block. Once the slice is dropped, the reference count decreases. When
// //! there are no more references to a block, it is considered "free" and may be
// //! reused. Additionally, the allocator keeps track of how many free blocks
// //! there are in total.
// //!
// //! The allocator uses a "scanning" allocating strategy, where it hands out
// //! slices into the current block until it is full, at which point it will scan
// //! the `refmap` forward for the next free block, using it for the next
// //! allocation. Once it reaches the end of the entire memory region, it wraps
// //! around to 0, and scans again from that point. If all blocks are in use, it
// //! allocates a new memory region.
// //!
// //! This allocator design is driven by two assumptions about the memory
// //! allocation patterns of the library:
// //! - Allocations always fit inside a single block.
// //! - Allocations always live for the same, relatively short amount of time.
// //!
// //! *Relatively short amount of time* here means enough time that the start of
// //! the memory region will be completely free once the allocator reaches the end
// //! of the memory region. A large enough memory region will behave as a kind of
// //! ring buffer.
// //!
// //! The goals of the allocator are:
// //! - Enable recycling memory
// //! - Prevent memory fragmentation
// //! - Provide cheap allocations

// use std::alloc::alloc_zeroed;
// use std::alloc::Layout;
// use std::rc::Rc;

// pub struct Allocator {
//   options: Options,
//   current_region: Rc<Region>,
// }

// impl Allocator {
//   pub fn new(options: Options) -> Result<Self, Error> {
//     options.validate()?;
//     let region = Region::new(options)?;
//     Ok(Self {
//       options,
//       current_region: region,
//     })
//   }
// }

// #[derive(Clone, Copy)]
// pub struct Options {
//   /// The size of blocks which the memory region will be divided into.
//   ///
//   /// Must be a power of two smaller than `region_size`.
//   pub block_size: usize,
//   /// The size of the memory region.
//   ///
//   /// Must be a power of two.
//   pub region_size: usize,
// }

// impl Options {
//   fn validate(&self) -> Result<(), Error> {
//     if !self.block_size.is_power_of_two() {
//       return Err(Error::InvalidOption("`block_size` must be a power of two"));
//     }
//     if !self.region_size.is_power_of_two() {
//       return Err(Error::InvalidOption("`region_size` must be a power of two"));
//     }
//     if self.region_size < self.block_size {
//       return Err(Error::InvalidOption("`region_size` must be a power of two"));
//     }

//     Ok(())
//   }
// }

// struct Region {
//   /// Index of the current block.
//   current_block_index: usize,
//   /// Offset into the current block.
//   current_block_offset: usize,
//   /// Size of each block.
//   block_size: usize,
//   /// layout:
//   ///
//   /// ```no_run
//   ///   refmap
//   ///   |
//   ///   v----------------------------v
//   /// [ [u8; region_size / block_size] [[u8; block_size]; region_size / block_size] ]
//   ///                                  ^------------------------------------------^
//   ///                                  |
//   ///                                  blocks
//   /// ```
//   data: Box<[u8]>,
// }

// impl Region {
//   fn new(options: Options) -> Result<Rc<Self>, Error> {
//     let data = unsafe {
//       alloc_zeroed(
//         Layout::array::<u8>(options.region_size).map_err(|_| Error::InvalidRegionLayout)?,
//       )
//     };
//     if data.is_null() {
//       return Err(Error::AllocationFailed);
//     }
//     let data = std::ptr::slice_from_raw_parts_mut(data, options.region_size);
//     let data = unsafe { Box::from_raw(data) };

//     Ok(Rc::new(Region {
//       current_block_index: 0,
//       current_block_offset: 0,
//       block_size: options.block_size,
//       data,
//     }))
//   }

//   fn refmap_len(&self) -> usize {
//     self.data.len() / self.block_size
//   }

//   fn refmap(&mut self) -> *mut [u8] {
//     self.data.
//     &mut (&mut *self.data)[0..self.refmap_len()]
//   }

//   fn data(&mut self) -> *mut [u8] {
//     &mut (&mut *self.data)[0..self.refmap_len()]
//   }

//   fn remaining_block_space(&self) -> usize {
//     self.block_size - self.current_block_offset
//   }

//   fn alloc(len: usize) -> Result<Slice, Error> {
//     // current_block is free?
//     // yes: keep it
//     // no: scan for next free block
//     //
//     // if no free block can be found,
//     // return Ok(None) and let allocator allocate a new region
//     //
//     // increment current block refcount, increment current_block_offset by len,
//     // return Slice
//   }
// }

// pub struct Slice {
//   region: Rc<Region>,
//   ptr: *mut [u8]
// }

// #[derive(Clone, Copy, Debug)]
// pub enum Error {
//   AllocationFailed,
//   InvalidRegionLayout,
//   InvalidOption(&'static str),
// }

// impl std::fmt::Display for Error {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match *self {
//       Error::AllocationFailed => write!(f, "failed to allocate region"),
//       Error::InvalidRegionLayout => write!(f, "failed to create region layout"),
//       Error::InvalidOption(desc) => write!(f, "invalid option: {desc}"),
//     }
//   }
// }

// impl std::error::Error for Error {}
