// SPDX-License-Identifier: Apache-2.0

use super::super::types::Argv;
use super::types::{CommittedSockaddrOutput, SockaddrOutput, StagedSockaddrOutput};
use super::Alloc;
use crate::guest::alloc::{Allocator, Collect, Collector, Stage};
use crate::{Result, NULL};

use libc::{c_int, c_long};

pub struct Accept4<T> {
    pub sockfd: c_int,
    pub addr: Option<T>,
    pub flags: c_int,
}

unsafe impl<'a, T: Into<SockaddrOutput<'a>>> Alloc<'a> for Accept4<T> {
    const NUM: c_long = libc::SYS_accept4;

    type Argv = Argv<4>;
    type Ret = c_int;

    type Staged = Option<StagedSockaddrOutput<'a>>;
    type Committed = Option<CommittedSockaddrOutput<'a>>;
    type Collected = Result<c_int>;

    fn stage(self, alloc: &mut impl Allocator) -> Result<(Self::Argv, Self::Staged)> {
        let addr = self.addr.map(Into::into).stage(alloc)?;
        let (addr_offset, addrlen_offset) = addr
            .as_ref()
            .map_or((NULL, NULL), |StagedSockaddrOutput { addr, addrlen }| {
                (addr.offset(), addrlen.offset())
            });
        Ok((
            Argv([
                self.sockfd as _,
                addr_offset,
                addrlen_offset,
                self.flags as _,
            ]),
            addr,
        ))
    }

    fn collect(
        addr: Self::Committed,
        ret: Result<Self::Ret>,
        col: &impl Collector,
    ) -> Self::Collected {
        addr.collect(col);
        ret
    }
}
