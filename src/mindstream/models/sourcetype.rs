#[derive(Debug, PartialEq, ToSql, FromSql, GraphQLEnum, Serialize, Deserialize)]
#[postgres(name = "sourcetype")]
pub enum SourceType {
    Rss,
    Twitter,
}

pub struct SourceTypeMapper;

mod impls_for_insert_and_query {
    use diesel::Queryable;
    use diesel::expression::AsExpression;
    use diesel::expression::bound::Bound;
    use diesel::pg::Pg;
    use diesel::row::Row;
    use diesel::types::*;
    use std::error::Error;
    use std::io::Write;

    use super::{SourceType, SourceTypeMapper};

    impl HasSqlType<SourceTypeMapper> for Pg {
        fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
            lookup.lookup_type("sourcetype")
        }
    }

    impl NotNull for SourceTypeMapper {}
    impl SingleValue for SourceTypeMapper {}

    impl<'a> AsExpression<SourceTypeMapper> for &'a SourceType {
        type Expression = Bound<SourceTypeMapper, &'a SourceType>;

        fn as_expression(self) -> Self::Expression {
            Bound::new(self)
        }
    }

    impl ToSql<SourceTypeMapper, Pg> for SourceType {
        fn to_sql<W: Write>(
            &self,
            out: &mut ToSqlOutput<W, Pg>,
        ) -> Result<IsNull, Box<Error + Send + Sync>> {
            match *self {
                SourceType::Rss => out.write_all(b"Rss")?,
                SourceType::Twitter => out.write_all(b"Twitter")?,
            }
            Ok(IsNull::No)
        }
    }

    impl FromSqlRow<SourceTypeMapper, Pg> for SourceType {
        fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
            match row.take() {
                Some(b"Rss") => Ok(SourceType::Rss),
                Some(b"Twitter") => Ok(SourceType::Twitter),
                Some(_) => Err("Unrecognized enum variant".into()),
                None => Err("Unexpected null for non-null column".into()),
            }
        }
    }

    impl Queryable<SourceTypeMapper, Pg> for SourceType {
        type Row = Self;

        fn build(row: Self::Row) -> Self {
            row
        }
    }
}
