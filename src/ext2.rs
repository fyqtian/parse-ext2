use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub struct Ext2 {
    pub image_path: String,
    pub fd: File,
}
impl Ext2 {
    pub fn new(image_path: &str) -> Self {
        let fd = File::open(image_path).unwrap();
        Ext2 {
            image_path: image_path.into(),
            fd,
        }
    }

    pub fn parse(&mut self) {
        let super_block = self.read_superblock().unwrap();
        println!("{:?}", super_block);
        let block_group_count = super_block
            .s_blocks_count
            .div_ceil(super_block.s_blocks_per_group) as usize;
        println!("Block group count: {}", block_group_count);
        let group_desc = self.read_group_descriptor(block_group_count).unwrap();
        println!("{:?}", group_desc);
    }

    pub fn read_superblock(&mut self) -> io::Result<crate::layout::Ext2SuperBlock> {
        self.fd
            .seek(SeekFrom::Start(crate::layout::SUPERBLOCK_OFFSET))?;

        let mut buffer = [0u8; crate::layout::SUPERBLOCK_SIZE];
        self.fd.read_exact(&mut buffer)?;

        let mut cursor = std::io::Cursor::new(buffer);
        let superblock = crate::layout::Ext2SuperBlock {
            s_inodes_count: cursor.read_u32::<LittleEndian>()?,
            s_blocks_count: cursor.read_u32::<LittleEndian>()?,
            s_r_blocks_count: cursor.read_u32::<LittleEndian>()?,
            s_free_blocks_count: cursor.read_u32::<LittleEndian>()?,
            s_free_inodes_count: cursor.read_u32::<LittleEndian>()?,
            s_first_data_block: cursor.read_u32::<LittleEndian>()?,
            s_log_block_size: cursor.read_u32::<LittleEndian>()?,
            s_log_frag_size: cursor.read_u32::<LittleEndian>()?,
            s_blocks_per_group: cursor.read_u32::<LittleEndian>()?,
            s_frags_per_group: cursor.read_u32::<LittleEndian>()?,
            s_inodes_per_group: cursor.read_u32::<LittleEndian>()?,
            s_mtime: cursor.read_u32::<LittleEndian>()?,
            s_wtime: cursor.read_u32::<LittleEndian>()?,
            s_mnt_count: cursor.read_u16::<LittleEndian>()?,
            s_max_mnt_count: cursor.read_u16::<LittleEndian>()?,
            s_magic: cursor.read_u16::<LittleEndian>()?,
            s_state: cursor.read_u16::<LittleEndian>()?,
            s_errors: cursor.read_u16::<LittleEndian>()?,
            s_minor_rev_level: cursor.read_u16::<LittleEndian>()?,
            s_lastcheck: cursor.read_u32::<LittleEndian>()?,
            s_checkinterval: cursor.read_u32::<LittleEndian>()?,
            s_creator_os: cursor.read_u32::<LittleEndian>()?,
            s_rev_level: cursor.read_u32::<LittleEndian>()?,
            s_def_resuid: cursor.read_u16::<LittleEndian>()?,
            s_def_resgid: cursor.read_u16::<LittleEndian>()?,
        };

        Ok(superblock)
    }
    pub fn read_group_descriptor(
        &mut self,
        c: usize,
    ) -> io::Result<Vec<crate::layout::Ext2GroupDesc>> {
        self.fd
            .seek(SeekFrom::Start(crate::layout::GROUP_DESC_OFFSET))?;

        let mut buffer = [0u8; crate::layout::GROUP_DESC_SIZE];
        self.fd.read_exact(&mut buffer)?;
        let mut cursor = std::io::Cursor::new(buffer);
        let mut descriptors = Vec::with_capacity(c as usize);
        for i in 0..c {
            let group_desc = crate::layout::Ext2GroupDesc {
                bg_block_bitmap: cursor.read_u32::<LittleEndian>()?,
                bg_inode_bitmap: cursor.read_u32::<LittleEndian>()?,
                bg_inode_table: cursor.read_u32::<LittleEndian>()?,
                bg_free_blocks_count: cursor.read_u16::<LittleEndian>()?,
                bg_free_inodes_count: cursor.read_u16::<LittleEndian>()?,
                bg_used_dirs_count: cursor.read_u16::<LittleEndian>()?,
                bg_pad: cursor.read_u16::<LittleEndian>()?,
                bg_reserved: [
                    cursor.read_u32::<LittleEndian>()?,
                    cursor.read_u32::<LittleEndian>()?,
                    cursor.read_u32::<LittleEndian>()?,
                ],
            };
            descriptors.push(group_desc);
        }
        Ok(descriptors)
    }

    pub fn read_inode_table(&mut self) -> io::Result<crate::layout::Ext2Inode> {}
}