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
