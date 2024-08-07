interface Window {
  /** NProgress instance */
  NProgress?: import('nprogress').NProgress
  /** Loading bar instance */
  $loadingBar?: import('naive-ui').LoadingBarProviderInst
  /** Dialog instance */
  $dialog?: import('naive-ui').DialogProviderInst
  /** Message instance */
  $message?: import('naive-ui').MessageProviderInst
  /** Notification instance */
  $notification?: import('naive-ui').NotificationProviderInst
}

namespace Calculate {
  interface Build {
    code: string
    level: 1 | 2 | 3
    need_value?: number
    value?: number
  }

  interface StoneBuild {
    buff_1: {
      code: string
      value: number
    }
    buff_2: {
      code: string
      value: number
    }
    // debuff: {
    //   code: string
    //   value: number
    // }
  }

  interface SelfBuild {
    buff_1: {
      code: string
      value: 3 | 6 | 9 | 12
    }
    buff_2: {
      code: string
      value: 3 | 6 | 9 | 12
    }
  }

  interface CalculatePageParam {
    need_builds: Build[]
    stone_builds: StoneBuild
    self_builds: SelfBuild
  }

  interface CalculateResult {
    result_array: string[]
    total_used_accessory_array: string[]
  }

  interface CalculatePriceParam {
    accessory_name: string
    accessory: string & 'amulet' | 'earring' | 'ring'
    build_string: string
    build: Record<string, number>
    is_artifact: 0 | 1
    is_artifact_disabled: boolean
    price: number
    is_buy: boolean
    base_string: string
    remark: string
  }

  interface CalculatePriceResult {
    used_items: CalculatePriceParam[]
    price_total: number
  }

  interface CalculatePriceResultTableColumn {
    amulet: CalculatePriceParam
    earring_1: CalculatePriceParam
    earring_2: CalculatePriceParam
    ring_1: CalculatePriceParam
    ring_2: CalculatePriceParam
    price_total: number
  }

  interface CalculateJson {
    calculatePageParam: CalculatePageParam
    artifactCheck: boolean
    data: CalculatePriceParam[]
    resultArray: CalculateResult['result_array']
  }
}
