# Query execution

Our example query on the Reddit dataset is:

{{code_block('user-guide/lazy/execution','df',['scan_csv'])}}

If we were to run the code above on the Reddit CSV the query would not be evaluated. Instead Polars
takes each line of code, adds it to the internal query graph and optimizes the query graph.

When we execute the code Polars executes the optimized query graph by default.

### Execution on the full dataset

We can execute our query on the full dataset by calling the `.collect` method on the query.

{{code_block('user-guide/lazy/execution','collect',['scan_csv','collect'])}}

```text
shape: (14_029, 6)
┌─────────┬───────────────────────────┬─────────────┬────────────┬───────────────┬────────────┐
│ id      ┆ name                      ┆ created_utc ┆ updated_on ┆ comment_karma ┆ link_karma │
│ ---     ┆ ---                       ┆ ---         ┆ ---        ┆ ---           ┆ ---        │
│ i64     ┆ str                       ┆ i64         ┆ i64        ┆ i64           ┆ i64        │
╞═════════╪═══════════════════════════╪═════════════╪════════════╪═══════════════╪════════════╡
│ 6       ┆ TAOJIANLONG_JASONBROKEN   ┆ 1397113510  ┆ 1536527864 ┆ 4             ┆ 0          │
│ 17      ┆ SSAIG_JASONBROKEN         ┆ 1397113544  ┆ 1536527864 ┆ 1             ┆ 0          │
│ 19      ┆ FDBVFDSSDGFDS_JASONBROKEN ┆ 1397113552  ┆ 1536527864 ┆ 3             ┆ 0          │
│ 37      ┆ IHATEWHOWEARE_JASONBROKEN ┆ 1397113636  ┆ 1536527864 ┆ 61            ┆ 0          │
│ …       ┆ …                         ┆ …           ┆ …          ┆ …             ┆ …          │
│ 1229384 ┆ DSFOX                     ┆ 1163177415  ┆ 1536497412 ┆ 44411         ┆ 7917       │
│ 1229459 ┆ NEOCARTY                  ┆ 1163177859  ┆ 1536533090 ┆ 40            ┆ 0          │
│ 1229587 ┆ TEHSMA                    ┆ 1163178847  ┆ 1536497412 ┆ 14794         ┆ 5707       │
│ 1229621 ┆ JEREMYLOW                 ┆ 1163179075  ┆ 1536497412 ┆ 411           ┆ 1063       │
└─────────┴───────────────────────────┴─────────────┴────────────┴───────────────┴────────────┘
```

Above we see that from the 10 million rows there are 14,029 rows that match our predicate.

With the default `collect` method Polars processes all of your data as one batch. This means that
all the data has to fit into your available memory at the point of peak memory usage in your query.

!!! warning "Reusing `LazyFrame` objects"

    Remember that `LazyFrame`s are query plans i.e. a promise on computation and is not guaranteed to cache common subplans. This means that every time you reuse it in separate downstream queries after it is defined, it is computed all over again. If you define an operation on a `LazyFrame` that doesn't maintain row order (such as a `group_by`), then the order will also change every time it is run. To avoid this, use `maintain_order=True` arguments for such operations.

### Execution on larger-than-memory data

If your data requires more memory than you have available Polars may be able to process the data in
batches using _streaming_ mode. To use streaming mode you simply pass the `engine="streaming"`
argument to `collect`

{{code_block('user-guide/lazy/execution','stream',['scan_csv','collect'])}}

### Execution on a partial dataset

While you're writing, optimizing or checking your query on a large dataset, querying all available
data may lead to a slow development process.

Instead, you can scan a subset of your partitions or use `.head`/`.collect` at the beginning and end
of your query, respectively. Keep in mind that the results of aggregations and filters on subsets of
your data may not be representative of the result you would get on the full data.

{{code_block('user-guide/lazy/execution','partial',['scan_csv','collect','head'])}}

```text
shape: (1, 6)
┌─────┬─────────────────────────┬─────────────┬────────────┬───────────────┬────────────┐
│ id  ┆ name                    ┆ created_utc ┆ updated_on ┆ comment_karma ┆ link_karma │
│ --- ┆ ---                     ┆ ---         ┆ ---        ┆ ---           ┆ ---        │
│ i64 ┆ str                     ┆ i64         ┆ i64        ┆ i64           ┆ i64        │
╞═════╪═════════════════════════╪═════════════╪════════════╪═══════════════╪════════════╡
│ 6   ┆ TAOJIANLONG_JASONBROKEN ┆ 1397113510  ┆ 1536527864 ┆ 4             ┆ 0          │
└─────┴─────────────────────────┴─────────────┴────────────┴───────────────┴────────────┘
```

## Diverging queries

It is very common that a query diverges at one point. In these cases it is recommended to use
`collect_all` as they will ensure that diverging queries execute only once.

```python
# Some expensive LazyFrame
lf: LazyFrame

lf_1 = LazyFrame.select(pl.all().sum())

lf_2 = lf.some_other_computation()

pl.collect_all([lf_1, lf_2]) # this will execute lf only once!
```
