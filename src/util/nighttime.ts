/**
 * 检查当前时间是否在夜晚时间范围内（晚上 8 点到早晨 6 点）
 * @returns {boolean} 当前时间是否在夜晚时间范围内
 */
export function isNighttime() {
  const now = new Date();
  const currentHour = now.getHours();

  // 定义夜晚时间范围
  const startHour = 20; // 晚上 8 点
  const endHour = 6; // 早晨 6 点

  // 检查时间范围
  if (startHour < endHour) {
    // 夜晚时间段不跨午夜
    return currentHour >= startHour && currentHour < endHour;
  } else {
    // 夜晚时间段跨午夜
    return currentHour >= startHour || currentHour < endHour;
  }
}
