create-image:
	dd if=/dev/zero of=ext2_image.img bs=1M count=100
	mkfs.ext2 ext2_image.img

mount:
	mkdir -p /mnt/ext2_image
	sudo mount -o loop ext2_image.img /mnt/ext2_image

umount:
	sudo umount /mnt/ext2_image