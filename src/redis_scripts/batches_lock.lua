local key = KEYS[1]
local field = KEYS[2]
local start_utsms = tonumber(ARGV[1])
local lock_milli_second = tonumber(ARGV[2])
local value = redis.call('HGET', key, field)
if value then
  local current_start_utsms = tonumber(value)
  if current_start_utsms + lock_milli_second < start_utsms then
    redis.call('HSET', key, field, start_utsms)
    return "1"
  end
else
  redis.call('HSET', key, field, start_utsms)
  return "1"
end