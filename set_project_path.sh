#!/bin/bash
sed -i -- 's?ABSOLUTE_PATH?'`pwd`'?' .cargo/config
