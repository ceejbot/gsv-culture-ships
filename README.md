# General Systems Vehicles Culture Ships

In case you ever needed Iain M. Banks's Culture ship names as a service. Names sourced from the pleasingly extensive [Wikipedia article](https://en.wikipedia.org/wiki/List_of_spacecraft_in_the_Culture_series) listing them. Sadly a closed set.

## Usage

```rust
let shipname = gsv_culture_ships::random();
println!("Out-of-context problem observed by {}.", shipname);
```

If you are using the cli bin:

```text
>  culture-ship
GSV Unreliable Witness
```

The `noncanonical` crate feature adds 5 ship names that are funny at the start of 2026, when AI is not, shall we say, at the Culture's level. They are, however, non-canonical, so by default they are not included.

## See also

[Javascript](https://github.com/ceejbot/culture-ships) and [golang](https://github.com/ceejbot/vfp-culture-ships) variations.

## LICENSE

[Blue Oak Model License](https://blueoakcouncil.org/license/1.0.0); text in [LICENSE.md](./LICENSE.md).
