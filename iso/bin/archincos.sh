#!/bin/bash

# Usage: ./archincos.sh <root_partition> <boot_partition> [swap_partition]

ROOT="$1"
BOOT="$2"
SWAP="$3"

echo "ROOT='$1', BOOT='$2', SWAP='$3' ($# args)"

if [ -z "$ROOT" ] || [ -z "$BOOT" ]; then
    echo "Usage: $0 <root_partition> <boot_partition> [swap_partition]"
    exit 1
fi

echo "Formatting root partition ($ROOT) as ext4..."
mkfs.ext4 "$ROOT"

echo "Formatting boot partition ($BOOT) as FAT32..."
mkfs.fat -F32 "$BOOT"

if [ -n "$SWAP" ]; then
    echo "Formatting swap partition ($SWAP) as swap..."
    mkswap "$SWAP"
fi

echo "Mounting root partition ($ROOT) at /mnt..."
mount "$ROOT" /mnt

echo "Creating /mnt/boot directory..."
mkdir -p /mnt/boot

echo "Mounting boot partition ($BOOT) at /mnt/boot..."
mount "$BOOT" /mnt/boot

if [ -n "$SWAP" ]; then
    echo "Enabling swap partition ($SWAP)..."
    swapon "$SWAP"
fi

set -e

if grep -qi 'amd' /proc/cpuinfo; then
    UCODE="amd-ucode"
else
    UCODE="intel-ucode"
fi

echo "Detected microcode: $UCODE"

read -p "Enter your username: " USERNAME
read -p "Enter your computer name (hostname): " COMPUTERNAME

LOCALE=$(cat /usr/share/i18n/SUPPORTED | fzf | cut -d ' ' -f1)
echo "Selected locale: $LOCALE"

BOOTLOADER_NAME="cynageOS"

echo "Pacstrapping base system..."
pacstrap -i /mnt base base-devel linux linux-firmware git sudo htop $UCODE nano vim bluez bluez-utils networkmanager

echo "Generating fstab..."
genfstab -U /mnt >> /mnt/etc/fstab
echo "Resulting /mnt/etc/fstab:"
cat /mnt/etc/fstab

cat <<EOF > /mnt/root/chroot_setup.sh
#!/bin/bash
set -e

echo "Setting root password..."
passwd
echo "Adding user $USERNAME..."
useradd -m -g users -G wheel,storage,power,video,audio -s /bin/bash $USERNAME
echo "Setting password for $USERNAME..."
passwd $USERNAME

echo "Editing sudoers..."
sed -i '/%wheel ALL=(ALL:ALL) ALL/s/^# //' /etc/sudoers

echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers

echo "Switching to user $USERNAME..."
su - $USERNAME -c 'sudo pacman -Syu'

echo "Configuring timezone..."
ZONE=\$(find /usr/share/zoneinfo/ -type f | sed 's|/usr/share/zoneinfo/||' | fzf)
ln -sf "/usr/share/zoneinfo/\$ZONE" /etc/localtime

echo "Setting hardware clock..."
hwclock --systohc

echo "Selecting locale..."
echo "$LOCALE UTF-8" >> /etc/locale.gen
locale-gen

echo "Setting locale..."
echo "LANG=$LOCALE" > /etc/locale.conf

echo "Setting hostname..."
echo "$COMPUTERNAME" > /etc/hostname

echo "Configuring /etc/hosts..."
cat <<EOT > /etc/hosts
127.0.0.1   localhost
::1         localhost
127.0.1.1   $COMPUTERNAME.localdomain $COMPUTERNAME
EOT

echo "Installing bootloader tools..."
pacman -S grub efibootmgr dosfstools mtools os-prober --noconfirm
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=$BOOTLOADER_NAME

echo "Enabling OS prober..."
echo "GRUB_DISABLE_OS_PROBER=false" >> /etc/default/grub

echo "Generating GRUB config..."
grub-mkconfig -o /boot/grub/grub.cfg

echo "Enabling system services..."
systemctl enable bluetooth
systemctl enable NetworkManager

echo "Finished inside chroot."
sed -i '/%wheel ALL=(ALL:ALL) NOPASSWD: ALL/s/^/# /' /etc/sudoers
EOF

chmod +x /mnt/root/chroot_setup.sh

echo "Entering chroot to complete setup..."
arch-chroot /mnt /root/chroot_setup.sh

echo "Cleaning chroot setup script..."
rm /mnt/root/chroot_setup.sh

echo "Unmounting and shutting down..."
umount -lR /mnt
shutdown now
