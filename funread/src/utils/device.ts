// 判断是否为移动端
export const isMobile = () => {
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
}

// 判断是否为桌面端
export const isDesktop = () => {
  return !isMobile()
}
