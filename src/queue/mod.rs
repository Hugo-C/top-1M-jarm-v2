use redis::{Commands, Connection, RedisResult};


const TASK_QUEUE: &str = "tranco:tasks";
const RESULT_QUEUE: &str = "tranco:results";
const REDIS_BLOCK_TIMEOUT: usize = 1;

pub(crate) struct Task {
    pub rank: String,
    pub domain: String,
}

pub(crate) fn push_task(con: &mut Connection, task: Task) -> RedisResult<()> {
    let task_value = format!("{};{}", task.rank, task.domain);
    con.lpush(TASK_QUEUE, task_value)
}

pub(crate) fn get_task(con: &mut Connection) -> Option<Task> {
    let message: RedisResult<Vec<String>> = con.brpop(TASK_QUEUE, REDIS_BLOCK_TIMEOUT);
    let vec_values = message.unwrap_or_default();  // TODO do not unwrap ?
    if vec_values.is_empty() {
        return None;
    }
    let [_queue, task_value] = &vec_values[..] else { panic!("unexpected task format!") };  // TODO return Result
    let [rank, domain] = task_value.split(';').collect::<Vec<&str>>()[..] else { panic!("unexpected task value format!") };
    Some(Task {
        rank: rank.to_string(),
        domain: domain.to_string(),
    })
}

pub(crate) fn nb_tasks(con: &mut Connection) -> u64 {
    con.llen(TASK_QUEUE).unwrap()  // TODO handle unwrap
}

pub(crate) struct JarmResult {
    pub rank: String,
    pub domain: String,
    pub jarm_hash: String,
}

pub(crate) fn push_jarm_result(con: &mut Connection, jarm_result: JarmResult) -> RedisResult<()> {
    let jarm_result_value = format!("{};{};{}", jarm_result.rank, jarm_result.domain, jarm_result.jarm_hash);
    con.lpush(RESULT_QUEUE, jarm_result_value)
}

pub(crate) fn get_jarm_result(con: &mut Connection) -> Option<JarmResult> {
    let message: RedisResult<Vec<String>> = con.brpop(RESULT_QUEUE, REDIS_BLOCK_TIMEOUT);
    let vec_values = message.unwrap_or_default();  // TODO do not unwrap ?
    if vec_values.is_empty() {
        return None;
    }
    let [_queue, jarm_result_value] = &vec_values[..] else { panic!("unexpected task format!") };  // TODO return Result
    let [rank, domain, jarm_hash] = jarm_result_value.split(';').collect::<Vec<&str>>()[..] else { panic!("unexpected jarm result value format!") };
    Some(JarmResult {
        rank: rank.to_string(),
        domain: domain.to_string(),
        jarm_hash: jarm_hash.to_string()
    })
}
