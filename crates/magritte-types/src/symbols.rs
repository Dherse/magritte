//! # Symbols
//! Symbol table implementation and traits for the Vulkan code generator.

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    mem::replace,
    ops::{Index, Range, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// An order preserving map that gets its key from the [`SymbolName`] trait.
#[derive(Clone, PartialEq, Eq)]
pub struct SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    values: Vec<T>,
    symbols: HashMap<Cow<'a, str>, usize>,
    symbols_pretty: HashMap<String, usize>,
}

impl<'a, 'de: 'a, T: SymbolName<'a>> Deserialize<'de> for SymbolTable<'a, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values = Vec::<T>::deserialize(deserializer)?;

        Ok(Self::from_vec(values))
    }
}

impl<'a, T: SymbolName<'a>> Serialize for SymbolTable<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.values.serialize(serializer)
    }
}

impl<'a, T> Debug for SymbolTable<'a, T>
where
    T: SymbolName<'a> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.values.fmt(f)
    }
}

impl<'a, T> Default for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    /// Creates a new symbol table
    #[inline]
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            symbols: HashMap::new(),
            symbols_pretty: HashMap::new(),
        }
    }

    pub fn from_vec(values: Vec<T>) -> Self {
        let symbols = values
            .iter()
            .map(SymbolName::name)
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect();
        let symbols_pretty = values
            .iter()
            .map(SymbolName::pretty_name)
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect();

        Self {
            values,
            symbols,
            symbols_pretty,
        }
    }

    /// Creates a new symbol table with a predefined capacity
    #[inline]

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
            symbols: HashMap::with_capacity(cap),
            symbols_pretty: HashMap::with_capacity(cap),
        }
    }

    /// Creates a new symbol table from an iterator
    pub fn with_iter(inputs: impl Iterator<Item = T>) -> Self {
        let (min, max) = inputs.size_hint();
        let mut values = Vec::with_capacity(max.unwrap_or(min));
        let mut symbols = HashMap::with_capacity(max.unwrap_or(min));
        let mut symbols_pretty = HashMap::with_capacity(max.unwrap_or(min));
        for (index, input) in inputs.enumerate() {
            symbols.insert(input.name(), index);
            symbols_pretty.insert(input.pretty_name(), index);
            values.push(input);
        }

        Self {
            values,
            symbols,
            symbols_pretty,
        }
    }

    /// Gets the number of symbols in this table
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Reserves additional size in the symbol table.
    #[inline]
    pub fn reserve(&mut self, size: usize) {
        self.values.reserve(size);
        self.symbols.reserve(size);
        self.symbols_pretty.reserve(size);
    }

    /// Extends this table from an iterator, appending all of its elements
    /// to the table.
    pub fn extend(&mut self, inputs: impl Iterator<Item = T>) {
        let (min, max) = inputs.size_hint();

        self.symbols.reserve(max.unwrap_or(min));
        self.values.reserve(max.unwrap_or(min));
        self.symbols_pretty.reserve(max.unwrap_or(min));

        inputs.for_each(|input| {
            self.push(input);
        });
    }

    /// Inserts an element in the table.
    pub fn push(&mut self, input: T) -> (Option<T>, usize) {
        let index = self.len();

        let name = input.name();
        if let Some(index) = self.symbols.get(&name) {
            (Some(replace(&mut self.values[*index], input)), *index)
        } else {
            self.symbols.insert(input.name(), index);
            self.symbols_pretty.insert(input.pretty_name(), index);
            self.values.push(input);

            (None, index)
        }
    }

    /// Clears the symbol table.
    pub fn clear(&mut self) {
        self.values.clear();
        self.symbols.clear();
        self.symbols_pretty.clear();
    }

    /// Returns `true` if the table is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an iterator over the symbols
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    /// Returns an iterator over the symbol names
    #[inline]
    pub fn names(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.symbols.keys()
    }

    /// Returns an **ordered** iterator over the names and values
    #[inline]
    pub fn iter_pair(&self) -> impl Iterator<Item = (Cow<'a, str>, &T)> {
        self.values.iter().map(|value| (value.name(), value))
    }

    /// Returns a mutable iterator
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    /// Returns a mutable iterator over the names and values
    #[inline]
    pub fn iter_pair_mut(&mut self) -> impl Iterator<Item = (Cow<'a, str>, &mut T)> {
        self.values.iter_mut().map(|value| (value.name(), value))
    }

    /// Gets an element from its index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    /// Gets a mutable reference to an element from its index
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    /// Gets an element from its name
    #[inline]
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        let index = self.symbols.get(name)?;
        Some(unsafe { self.values.get(*index).unwrap_unchecked() })
    }

    /// Gets a mutable reference to an element from its name
    #[inline]
    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut T> {
        let index = self.symbols.get(name)?;
        Some(unsafe { self.values.get_mut(*index).unwrap_unchecked() })
    }

    /// Gets an element from its pretty name
    #[inline]
    pub fn get_by_pretty(&self, name: &str) -> Option<&T> {
        let index = self.symbols_pretty.get(name)?;
        Some(unsafe { self.values.get(*index).unwrap_unchecked() })
    }

    /// Gets a mutable reference to an element from its pretty name
    #[inline]
    pub fn get_by_pretty_mut(&mut self, name: &str) -> Option<&mut T> {
        let index = self.symbols_pretty.get(name)?;
        Some(unsafe { self.values.get_mut(*index).unwrap_unchecked() })
    }

    /// Gets an element from either its name or its pretty name
    #[inline]
    pub fn get_by_either(&self, name: &str) -> Option<&T> {
        self.get_by_name(name).or_else(|| self.get_by_pretty(name))
    }

    /// Gets an element from either its name or its pretty name
    #[inline]
    pub fn get_mut_by_either(&mut self, name: &str) -> Option<&mut T> {
        let index = self.symbols.get(name)?;

        self.values.get_mut(*index)
    }

    /// Checks whether the given name exists in the symbol table
    #[inline]
    pub fn contains_name(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Return the table as a slice of values
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values[..]
    }

    /// Gets a reference to the first element
    pub fn first(&self) -> Option<&'_ T> {
        self.values.first()
    }

    /// Gets a mutable reference to the first element
    pub fn first_mut(&mut self) -> Option<&'_ mut T> {
        self.values.first_mut()
    }

    /// Gets a reference to the last element
    pub fn last(&self) -> Option<&'_ T> {
        self.values.last()
    }

    /// Gets a mutable reference to the last element
    pub fn last_mut(&mut self) -> Option<&'_ mut T> {
        self.values.last_mut()
    }

    /// Does the symbol table contain an element with a certain key
    pub fn contains_key(&self, key: &str) -> bool {
        self.symbols.contains_key(key) || self.symbols_pretty.contains_key(key)
    }

    /// Updates the list of symbols of this table.
    pub fn update_symbols(&mut self) {
        self.symbols.clear();
        self.symbols_pretty.clear();

        for (i, value) in self.values.iter().enumerate() {
            self.symbols.insert(value.name().clone(), i);
            self.symbols_pretty.insert(value.pretty_name().clone(), i);
        }
    }

    /// Keeps only the item validated by the predicate
    pub fn retain<F: FnMut(&'_ T) -> bool>(&mut self, predicate: F) {
        self.values.retain(predicate);

        self.update_symbols();
    }
}

impl<'a: 'b, 'b, T> Index<&'b str> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = T;

    fn index(&self, index: &'b str) -> &Self::Output {
        match self.get_by_name(index) {
            Some(t) => t,
            None => panic!("could not find symbol with name: `{}`", index),
        }
    }
}

impl<'a, T> Index<usize> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(t) => t,
            None => panic!("index out of bounds: `{}`, len: `{}`", index, self.len()),
        }
    }
}

impl<'a, T> Index<RangeFull> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = [T];

    fn index(&self, _: RangeFull) -> &Self::Output {
        self.as_slice()
    }
}

impl<'a, T> Index<Range<usize>> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'a, T> Index<RangeInclusive<usize>> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = [T];

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'a, T> Index<RangeTo<usize>> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = [T];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'a, T> Index<RangeToInclusive<usize>> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Output = [T];

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'a, T> FromIterator<T> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        Self::with_iter(iter.into_iter())
    }
}

impl<'a: 'b, 'b, T> IntoIterator for &'b SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Item = &'b T;

    type IntoIter = std::slice::Iter<'b, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T> Extend<T> for SymbolTable<'a, T>
where
    T: SymbolName<'a>,
{
    fn extend<A: IntoIterator<Item = T>>(&mut self, inputs: A) {
        self.extend(inputs.into_iter());
    }
}

impl<'a: 'b, 'b, T: 'a> IntoParallelRefIterator<'b> for SymbolTable<'a, T>
where
    T: SymbolName<'a> + Send + Sync,
{
    type Iter = rayon::slice::Iter<'b, T>;

    type Item = &'b T;

    fn par_iter(&'b self) -> Self::Iter {
        self.values.par_iter()
    }
}

/// A type that has an associated symbol name
pub trait SymbolName<'a> {
    /// Gets the symbol name
    fn name(&self) -> Cow<'a, str>;

    /// The pretty name of this symbol
    fn pretty_name(&self) -> String;
}

impl<'a> SymbolName<'a> for Cow<'a, str> {
    #[inline]
    fn name(&self) -> Cow<'a, str> {
        self.clone()
    }

    #[inline]
    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}
