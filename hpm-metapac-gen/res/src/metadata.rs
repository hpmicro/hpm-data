pub mod ir {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct IR {
        pub blocks: &'static [Block],
        pub fieldsets: &'static [FieldSet],
        pub enums: &'static [Enum],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Block {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub items: &'static [BlockItem],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItem {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub array: Option<Array>,
        pub byte_offset: u32,

        pub inner: BlockItemInner,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BlockItemInner {
        Block(BlockItemBlock),
        Register(Register),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Register {
        pub access: Access,
        pub bit_size: u32,
        pub fieldset: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItemBlock {
        pub block: &'static str,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Access {
        ReadWrite,
        Read,
        Write,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FieldSet {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub fields: &'static [Field],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Field {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub bit_offset: BitOffset,
        pub bit_size: u32,
        pub array: Option<Array>,
        pub enumm: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Array {
        Regular(RegularArray),
        Cursed(CursedArray),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularArray {
        pub len: u32,
        pub stride: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedArray {
        pub offsets: &'static [u32],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BitOffset {
        Regular(RegularBitOffset),
        Cursed(CursedBitOffset),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularBitOffset {
        pub offset: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedBitOffset {
        pub ranges: &'static [core::ops::RangeInclusive<u32>],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Enum {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub variants: &'static [EnumVariant],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct EnumVariant {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub value: u64,
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub family: &'static str,
    pub memory: &'static [MemoryRegion],
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [Interrupt],
    pub dma_channels: &'static [DmaChannel],
    pub resources: &'static [Resource],
    pub clocks: &'static [Clock],
    pub pins: &'static [IoPin],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resource {
    pub name: &'static str,
    pub index: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clock {
    pub name: &'static str,
    pub index: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IoPin {
    pub name: &'static str,
    pub index: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub kind: MemoryRegionKind,
    pub address: u32,
    pub size: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MemoryRegionKind {
    Flash,
    Ram,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Interrupt {
    pub name: &'static str,
    pub number: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub name: &'static str,
    pub package: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub registers: Option<PeripheralRegisters>,
    pub sysctl: Option<PeripheralSysctl>,
    pub pins: &'static [PeripheralPin],
    pub dma_channels: &'static [PeripheralDmaChannel],
    pub interrupts: &'static [PeripheralInterrupt],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRegisters {
    pub kind: &'static str,
    pub version: &'static str,
    pub block: &'static str,
    pub ir: &'static ir::IR,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralInterrupt {
    pub signal: &'static str,
    pub interrupt: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralSysctl {
    pub group_link: usize,
    pub group_bit_offset: u8,
    pub resource_clock_top: Option<usize>,
    pub resource: usize,
    pub clock_node: Option<usize>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralPin {
    pub pin: &'static str,
    pub signal: &'static str,
    pub alt: Option<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DmaChannel {
    pub name: &'static str,
    pub dma: &'static str,
    pub channel: u32,
    pub dmamux_channel: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralDmaChannel {
    pub signal: &'static str,
    pub dmamux: Option<&'static str>,
    pub request: Option<u32>,
}
