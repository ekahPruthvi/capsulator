#!/bin/bash

mkdir -p $HOME/cynageiso
cp -r /usr/share/archiso/configs/releng $HOME/cynageiso/cos/
cp /etc/os-release $HOME/cynageiso/cos/airootfs/etc/os-release
