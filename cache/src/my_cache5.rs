// Like MyCache4 but with limited caching, so only store the last 4K positions or so under the assumption
// That you're unlikely to lookahead too much usually. For large files like 2MB cache sizes are in the
// hundreds of MB for MyCache4 so it's untenable eventually.
