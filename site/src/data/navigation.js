const navigation = [
  {
    label: { en: 'Start', zh: '开始' },
    items: [
      { slug: '', label: { en: 'Introduction', zh: '项目介绍' } },
      { slug: 'quick-start', label: { en: 'Quick Start', zh: '快速开始' } }
    ]
  },
  {
    label: { en: 'hikcamera', zh: 'hikcamera' },
    items: [
      { slug: 'hikcamera/lifecycle', label: { en: 'Lifecycle', zh: '生命周期' } },
      { slug: 'hikcamera/devices', label: { en: 'Devices', zh: '设备信息' } },
      { slug: 'hikcamera/camera', label: { en: 'Camera', zh: '相机控制' } },
      { slug: 'hikcamera/show', label: { en: 'Show', zh: '显示窗口' } },
      { slug: 'hikcamera/errors', label: { en: 'Error Handling', zh: '错误处理' } }
    ]
  },
  {
    slug: 'hikcamera-sys',
    label: { en: 'hikcamera-sys', zh: 'hikcamera-sys' }
  },
  {
    slug: 'design-philosophy',
    label: { en: 'Design Philosophy', zh: '设计哲学' }
  }
]

const navigationItems = navigation.flatMap(group => group.items ?? [group])

export { navigation, navigationItems }
