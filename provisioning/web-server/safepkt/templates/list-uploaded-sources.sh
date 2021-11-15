#!/bin/bash

cd /uploaded-sources || exit

# shellcheck disable=SC2012
ls ./ | sed -e 's/.rs.b64$//g'