while getopts f: flag
do
    case "${flag}" in
        f) args=${OPTARG};;
    esac
done

echo "Building Tenant Onboarding app..."
cargo build

echo "ext Flag: $args" 
echo "ext Flag: $3" 
cd target/debug
./tenant-onboarding $3 $args
 