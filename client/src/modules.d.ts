declare var Promise: any;

declare module '*.css' {
    interface ClassNames {
      [className: string]: string
    }
    const classNames: ClassNames
    export = classNames
  }
