interface Window {
  /** NProgress instance */
  NProgress?: import('nprogress').NProgress;
  /** Loading bar instance */
  $loadingBar?: import('naive-ui').LoadingBarProviderInst;
  /** Dialog instance */
  $dialog?: import('naive-ui').DialogProviderInst;
  /** Message instance */
  $message?: import('naive-ui').MessageProviderInst;
  /** Notification instance */
  $notification?: import('naive-ui').NotificationProviderInst;
}

namespace Calculate {
  interface Build {
    code: number | string
    level: 1 | 2 | 3
    need_value?: number
    value?: number
  }

  interface StoneBuild {
    buff_1: {
      code: number | string
      value: number
    },
    buff_2: {
      code: number | string
      value: number
    },
    // debuff: {
    //   code: number | string
    //   value: number
    // }
  }

  interface SelfBuild {
    buff_1: {
      code: number | string
      value: 3 | 6 | 9 | 12
    },
    buff_2: {
      code: number | string
      value: 3 | 6 | 9 | 12
    }
  }

  interface CalculatePageParam {
    need_builds: Calculate.Build[]
    stone_builds: Calculate.StoneBuild
    self_builds: Calculate.SelfBuild
  }
}
