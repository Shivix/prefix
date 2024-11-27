set -e

diff_tool="nvim -d "

arg_sets=(
    "--summary --tag"
    "--summary=55"
    "--summary=54 --value"
    "--summary" "54 some 55" "--only-fix"
    "--tag"
    "--value"
    "--strict"
    "--strict --only-fix"
    "--strip"
    "--delimiter=|"
    "--strip --delimiter=| --value --color=never"
)

msg_sets=(
    "8=FIX.4.4|35=A|34=1092|49=TESTBUY1|56=TESTSELL1|10=178|"
    "8=FIX.4.4^35=D^34=192^49=SENDER^56=TARGET^55=EURUSD^10=123^"
    "8=FIX.4.4|35=8|34=192|49=SENDER|56=TARGET|55=EURUSD|54=1"
    "8=FIX.4.4|35=8|34=192|49=SENDER|56=TARGET|55=EURUSD|54=1|17=12345678910|10=123|"
    "8=FIX.4.4|9=182|35=W|49=Sender|56=Target|34=2|52=20231126-12:34:56.789|55=EUR/USD|268=3|269=0|270=1.0950|271=1000000|269=1|270=1.0948|271=2000000|269=2|270=1.0952|271=500000|10=128|"
)

before_cmd="prefix"
after_cmd="target/release/prefix"

cargo build --release

before_file=$(mktemp)
after_file=$(mktemp)

cleanup() {
    rm -f "$before_file" "$after_file"
}
trap cleanup EXIT

for args in "${arg_sets[@]}"; do
    for msg in "${msg_sets[@]}"; do
        # Make it easier to see which command is different.
        echo "==========" $args $msg "==========">> "$before_file"
        echo "==========" $args $msg "==========" >> "$after_file"

        $before_cmd $msg $args >>"$before_file"
        $after_cmd $msg $args >>"$after_file"
    done
    cat test/test.txt | $before_cmd $args >>"$before_file"
    cat test/test.txt | $after_cmd $args >>"$after_file"
done
cat test/test.txt | $before_cmd --summary 55 | sort | uniq --count >>"$before_file"
cat test/test.txt | $after_cmd --summary 55 | sort | uniq --count >>"$after_file"

$diff_tool "$before_file" "$after_file"
