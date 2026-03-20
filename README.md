# Container talk (SRUG) 🍱
Code to accompany the SRUG (Seattle Rust User Group) talk from March 19, 2026 by Carlo Quick.

Presentation Slides: https://docs.google.com/presentation/d/1_7EwcKPybzNC_Qpz80lYa7p-9LG-ivGlpFoCjNTrVe0/edit?usp=drive_link  
LinkedIn: https://www.linkedin.com/in/carloquick/  
Bento: https://github.com/CarloQuick/bento

**Note**
> This demonstration uses the Linux kernel.
>
> You'll need to be on a Linux machine, Linux VM, use WSL2, or in a container with root privileges.

## Tutorial

Requires [Docker](https://www.docker.com/get-started/) to pull OCI-compliant images.

Create the directories and extract the busybox image into your `ROOTFS` directory:

```bash
mkdir -p /path/to/rootfs /path/to/container_dir
CONTAINER_ID=$(docker create busybox)
docker export $CONTAINER_ID | tar -xC /path/to/rootfs
docker rm $CONTAINER_ID
```
```bash
mv .env.example .env
```

In your `.env` file, you will find 2 entries. `ROOTFS` is the path to your extracted busybox filesystem and `CONTAINER_DIR` is where the rootfs will be bind-mounted when the container runs.

```sh
ROOTFS=
CONTAINER_DIR=
```

Then set `ROOTFS` and `CONTAINER_DIR` in your `.env` to those same paths.

By default, the container is rootless, so you can create containers without giving the process sudo permissions.

```bash
cargo run
```

This will isolate the process into its own **User**, **UTS**, **PID**, and **Mount** namespaces, bind-mount your rootfs into `CONTAINER_DIR`, chroot into it, and run `ls` inside the isolated environment. You should see hostname, PID, and UID printed before and after isolation.

## Current status

**Educational** — This is not production-ready and should not be used in production environments. It exists purely for educational purposes and to demonstrate understanding of container internals.