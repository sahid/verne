# A human virtual resources creator from templates

Currently only `libvirt` is supported as driver. It provided ability
create resource with QEMU, LXC and much more (see:
http://libvirt.org).

Make sure to have `libvirt-dev` or `libvirt-devel` package (or the
development files otherwise somewhere in your include path).

For using LXC, please consider to build libvirt with LXC or install
the related packages.

# Example

```
# cat > ~/simple.yaml <<EOF
guest:
  - name: myguest
    memory: 2048 # KiB
    vcpus: 1

  - name: yourguest
    memory: 2048 # KiB
    vcpus: 2
EOF

# verne create ~/simple.yaml
# verne clean ~/simple.yaml
```

In this example `Verne` is using the default driver `libvirt` and
default template parser `Yaml`.

The default connection used by libvirt is generally `qemu:///system`,
it will be possible to use an other connection by using the argument
`uri`.

```
# verne create ~/simple.yaml --uri test:///default
# verne clean ~/simple.yaml  --uri test:///default
```

An other solution to set the default `uri` is to use the environement
variable `LIBVIRT_DEFAULT_URI`.

Because `Verne` is using `libvirt` it will be possible to create LXC
containers.

```
# export LIBVIRT_DEFAULT_URI=lxc:///

# verne create ~/simple.yaml
# verne clean ~/simple.yaml
```
