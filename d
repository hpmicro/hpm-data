#!/bin/bash

set -ex

cd $(dirname $0)

CMD=$1
shift

case "$CMD" in
    download-all)
        git clone https://github.com/hpmicro/hpm_sdk.git
    ;;
    install-chiptool)
        cargo install --git https://github.com/ch32-rs/chiptool
    ;;
    extract-all)
        peri=$1
        shift
        echo $@

        rm -rf tmp/$peri
        mkdir -p tmp/$peri

        for f in `ls svd/HPM*`; do
            echo $f
            svd_path=$f
            f=${f#"svd/HPM"}
            f=${f%"_svd.xml"}
            echo -n processing $f ...
            if test -f  transforms/$peri.yaml; then
                trans_args="--transform transforms/$peri.yaml"
            fi

            if chiptool extract-peripheral $trans_args --svd $svd_path --peripheral $peri $@ > tmp/$peri/$f.yaml 2> tmp/$peri/$f.err; then
                rm tmp/$peri/$f.err
                echo OK
            else
                if grep -q 'peripheral not found' tmp/$peri/$f.err; then
                    echo No Peripheral
                else
                    echo OTHER FAILURE
                fi
                rm tmp/$peri/$f.yaml
            fi
        done
    ;;
    gen)
        rm -rf build/data
        cargo run --release -p hpm-data-gen && cargo run --release -p hpm-metapac-gen -- "HPM53*" "HPM67*" "HPM64*" "HPM62*" "HPM63*" "HPM68*" "HPM6E*"
    ;;
    ci)
        echo TODO $CMD
    ;;
    *)
        echo "unknown command"
    ;;
esac

