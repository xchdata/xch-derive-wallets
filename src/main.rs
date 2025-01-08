use bech32::ToBase32;
use chia::bls::PublicKey;
use chia::puzzles::{standard::StandardArgs, DeriveSynthetic};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None, args_conflicts_with_subcommands = true)]
struct Cli {
    #[arg(long, short = 'k')]
    pk: String,

    #[arg(long, short = 'i', default_value_t = 0)]
    index: u32,

    #[arg(long, short = 'n', default_value_t = 10)]
    count: u32,
}

fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    let pk_hexstr = args.pk;
    let pk_bytes = hex::decode(pk_hexstr)?;
    let pk = PublicKey::from_bytes(pk_bytes.as_slice().try_into()?)?;
    for i in args.index..args.count {
        let child_pk = chia::bls::master_to_wallet_unhardened(&pk, i);
        let child_pk_syn = child_pk.derive_synthetic();
        let ph = StandardArgs::curry_tree_hash(child_pk_syn);
        let ph_hexstr = hex::encode(ph);
        let addr = bech32::encode("xch", ph.to_base32(), bech32::Variant::Bech32m)?;
        println!("{}\t{}", ph_hexstr, addr);
    }

    Ok(())
}
