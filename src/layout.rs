#[repr(C)]
#[derive(Debug)]
pub struct Ext2SuperBlock {
    pub s_inodes_count: u32,
    pub s_blocks_count: u32,
    pub s_r_blocks_count: u32,
    pub s_free_blocks_count: u32,
    pub s_free_inodes_count: u32,
    pub s_first_data_block: u32,
    pub s_log_block_size: u32,
    pub s_log_frag_size: u32,
    pub s_blocks_per_group: u32,
    pub s_frags_per_group: u32,
    pub s_inodes_per_group: u32,
    pub s_mtime: u32,
    pub s_wtime: u32,
    pub s_mnt_count: u16,
    pub s_max_mnt_count: u16,
    pub s_magic: u16,
    pub s_state: u16,
    pub s_errors: u16,
    pub s_minor_rev_level: u16,
    pub s_lastcheck: u32,
    pub s_checkinterval: u32,
    pub s_creator_os: u32,
    pub s_rev_level: u32,
    pub s_def_resuid: u16,
    pub s_def_resgid: u16,
    // Other fields can be added as needed
}

#[repr(C)]
#[derive(Debug)]
pub struct Ext2GroupDesc {
    pub bg_block_bitmap: u32,
    pub bg_inode_bitmap: u32,
    pub bg_inode_table: u32,
    pub bg_free_blocks_count: u16,
    pub bg_free_inodes_count: u16,
    pub bg_used_dirs_count: u16,
    pub bg_pad: u16,
    pub bg_reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct Ext2Inode {
    pub i_mode: u16,
    pub i_uid: u16,
    pub i_size: u32,
    pub i_atime: u32,
    pub i_ctime: u32,
    pub i_mtime: u32,
    pub i_dtime: u32,
    pub i_gid: u16,
    pub i_links_count: u16,
    pub i_blocks: u32,
    pub i_flags: u32,
    pub i_osd1: [u32; 3],
    pub i_block: [u32; 15],
    pub i_generation: u32,
    pub i_file_acl: u32,
    pub i_dir_acl: u32,
    pub i_faddr: u32,
    pub i_osd2: [u32; 12],
}

#[repr(C)]
#[derive(Debug)]
pub struct Ext2DirEntry {
    pub inode: u32,
    pub rec_len: u16,
    pub name_len: u8,
    pub file_type: u8,
    pub name: [u8; 255], // Max name length
}

#[repr(C)]
#[derive(Debug)]
pub struct Inode {
    pub mode: u16,        // File mode
    pub uid: u16,         // Owner's user ID
    pub size: u32,        // Size in bytes
    pub atime: u32,       // Access time
    pub ctime: u32,       // Creation time
    pub mtime: u32,       // Modification time
    pub dtime: u32,       // Deletion time
    pub gid: u16,         // Group ID
    pub links_count: u16, // Number of links (directories)
    pub blocks: u32,      // Number of 512-byte blocks
    pub flags: u32,       // File flags
    pub osd1: u32,        // OS-dependent value
    pub block: [u32; 15], // Block pointers
    pub generation: u32,  // File version (NFS)
    pub file_acl: u32,    // File ACL
    pub dir_acl: u32,     // Directory ACL
    pub faddr: u32,       // Fragment address
    pub osd2: [u8; 12],   // OS-dependent value
}

#[derive(Debug)]
pub enum FileType {
    Unknown,
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    FIFO,
    Socket,
    SymbolicLink,
}

pub struct InodeTable {
    inodes: Vec<Inode>,
}

pub const SUPERBLOCK_OFFSET: u64 = 1024; // Superblock is located at 1024 bytes
pub const SUPERBLOCK_SIZE: usize = 1024; // Size of the superblock

pub const GROUP_DESC_OFFSET: u64 = 4096; // Group descriptor is located at 2048 bytes
pub const GROUP_DESC_SIZE: usize = 32; // Size of the group descriptor

pub const INODE_SIZE: usize = 128; // Size of each inode

pub const DIR_ENTRY_SIZE: usize = 32; // Size of each directory entry

pub const INODE_TABLE_OFFSET: u64 = 4096; // Inode table is located at 4096 bytes
