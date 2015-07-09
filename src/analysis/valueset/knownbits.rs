use super::{ValueSet, ScannableSet, KnownBits, UIntMultiple, UIntRange, SIntRange};
use std::ops::{BitAnd, BitOr};
use util::{blcic,bitsmear};

impl ValueSet<u64> for KnownBits {
	fn contains(&self, value: u64) -> bool {
		if self.zerobits & self.onebits != 0 {
			return false // pattern unfulfillable
		}
		value & (self.zerobits | self.onebits) == self.onebits
	}
}

fn scan_up(value: u64, zeroes: u64, ones: u64) -> Option<u64> {
	let fixedbits = zeroes | ones;
	if value & fixedbits == ones { return Option::Some(value) }

	let over = bitsmear(fixedbits & (ones ^ value));
	let bsm  = value & over; // value & !down;
	let increase = bitsmear(bsm) + 1;
	let rounded = ((value & !over) | fixedbits) + (increase & !over);
	let overwritten = (!fixedbits & rounded) | ones;

	Option::Some(overwritten)
}

impl ScannableSet<u64> for KnownBits {
	fn scan_up(&self, value: u64) -> Option<u64> {
		scan_up(value, self.zerobits, self.onebits)
	}
	fn scan_dn(&self, value: u64) -> Option<u64> {
		scan_up(!value, self.onebits, self.zerobits).and_then(|x| Option::Some(!x))
	}
}

impl KnownBits {
	pub fn as_umultiple(&self) -> UIntMultiple {
		let fixedbits = self.zerobits | self.onebits;
		let f_blcic = blcic(fixedbits);
		UIntMultiple {
			modulus: f_blcic,
			residue: (f_blcic - 1) & self.onebits
		}
	}
	pub fn as_urange(&self) -> UIntRange {
		let fixedbits = self.zerobits | self.onebits;
		UIntRange {
			min: (u64::min_value() & !fixedbits) | self.onebits,
			max: (u64::max_value() & !fixedbits) | self.onebits
		}
	}
	pub fn as_srange(&self) -> SIntRange {
		let fixedbits = self.zerobits | self.onebits;
		SIntRange {
			min: ((i64::min_value() as u64 & !fixedbits) | self.onebits) as i64,
			max: ((i64::max_value() as u64 & !fixedbits) | self.onebits) as i64
		}
	}
}

impl<'a, 'b> BitAnd<&'a KnownBits> for &'b KnownBits {
	type Output = KnownBits;

	fn bitand(self, rhs: &KnownBits) -> KnownBits {
		KnownBits {
			zerobits: self.zerobits | rhs.zerobits,
			onebits: self.onebits | rhs.onebits
		}
	}
}

impl<'a, 'b> BitOr<&'a KnownBits> for &'b KnownBits {
	type Output = KnownBits;

	fn bitor(self, rhs: &KnownBits) -> KnownBits {
		KnownBits {
			zerobits: self.zerobits & rhs.zerobits,
			onebits: self.onebits & rhs.onebits
		}
	}
}