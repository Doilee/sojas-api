/* Queries:
 * prepared (parameterized):
 *   have their quey plan cached, use a
 *   binary mode of communication (lower
 *   bandwith and faster decoding), and
 *   utilize parameters to avoid SQL
 *   Injection
 * unprepared (simple):
 *   intended only for use case where
 *   prepared  statement will not work
 * 
 * &str is treated as an unprepared query
 * Query or QueryAs struct is treated as
 * prepared query
 *
 *  conn.execute("BEGIN").await                            <- unprepared
 *  conn.execute(sqlx::query("DELETE FROM table")).await   <- prepared
 * 
 * .execute   <- returns number of affected rows, drops result 
 * receive results:
 * .fetch           <- the fetch query finalizer returns a stream-like
 *                     type that iterates through the rows in the result sets.
 * .fetch_one       <- fetch_one & fetch_optional to request one required or
 *                     optional result from the database
 * .fetch_optional  <-
 * .fetch_all
 * 
 * All methods accept one of &mut {connection type}, &mut Transaction or &Pool
 * 
 * sqlx::query  <- will return Row<'conn>
 *   Column values can be accessed with row.get()
 * 
 * 
 * Number of Rows   Method to call              Returns                                           Notes
 * None             .execute(...).await         sqlx::Result<DB::QueryResult>                     For INSERT/UPDATE/DELETE without RETURNING.
 * Zero or One      .fetch_optional(...).await  sqlx::Result<Option<{adhoc struct}>>              Extra rows are ignored.
 * Exactly One      .fetch_one(...).await       sqlx::Result<{adhoc struct}>                      Errors if no rows were returned. Extra rows are ignored. Aggregate queries, use this.
 * At Least One     .fetch(...)                 impl Stream<Item = sqlx::Result<{adhoc struct}>>  Call .try_next().await to get each row result.
 * Multiple         .fetch_all(...).await       sqlx::Result<Vec<{adhoc struct}>>	
 * 
 * 
 */
let pool: &MySqlPool = &*app_state.pool.lock().unwrap();

// TODO what is difference between fetch && fetch_all
// TODO fetch(&mut conn)
// TODO fetch_all(&pool)
// TODO what is fetch & and what is fetch_all
// TODO why does fetch need connection and why does fetch_all need pooled connection
// TODO fetch & fetch_all accepts &mut conn or &pool??

// TODO how to use fetch || how to use streams?
let users_0 = sqlx::query("SELECT * FROM users")
    .map(|row: MySqlRow| {
        // ...
        // TODO doesnt get called?
        println!("user_0 map:");
        println!("{:#?}", row);
        "test"
    })
    .fetch(pool); // returns Pin<Box<dyn futures_core::stream::Stream<Item = Result<User, sqlx::Error>> + Send>>

let users_1: Vec<MySqlRow> = sqlx::query("SELECT * FROM users")
    .fetch_all(pool)
    .await
    .unwrap();

println!("users_1: ");
println!("{:#?}", users_1);
println!("_____________________________________________________________________");

// query_as is like query but with typing casting
#[derive(sqlx::FromRow, Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
}

let users_2 = sqlx::query_as::<_, User>("SELECT * FROM users")
// .map(|user: User| {
// ..
// })
    .fetch(pool);  // returns Pin<Box<dyn futures_core::stream::Stream<Item = Result<User, sqlx::Error>> + Send>>

/* query! macro
 * to achieve compile-time syntactic and
 * semantic verification of the SQL
 * 
 * DATABASE_URL environment variable must be set at build time
 */
let users_3 = sqlx::query!(
    "SELECT * FROM users"
    // , binded values
).fetch_all(pool) // -> Vec<{ id: i64, username: String, email: String }>
    .await             //    The output type is an anonymous record.
    .unwrap(); // returns sqlx::Result<Vec<{adhoc struct}>>

println!("users_3: ");
println!("{:#?}", users_3);
println!("_____________________________________________________________________");


// named output type
let users_4: Vec<User> = sqlx::query_as!(
    User,
    "SELECT * FROM users",
    // , binded values
).fetch_all(pool) // -> Vec<User>
    .await
    .unwrap();

println!("users_4: ");
println!("{:#?}", users_4);
println!("_____________________________________________________________________");

