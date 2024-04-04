/* tslint:disable */
/* eslint-disable */
declare module 'resize_worker' {
  /**
   * Resizes the supplied ImageData rgba array.
   * @param {Uint8Array} data
   * @param {number} sourceWidth
   * @param {number} sourceHeight
   * @param {number} targetWidth
   * @param {number} targetHeight
   * @param {boolean} [hq=true]
   * @return {Promise<Uint8Array>}
   */
  export function resize(
    data: Uint8Array,
    sourceWidth: number, sourceHeight: number,
    targetWidth: number, targetHeight: number,
    hq: boolean
  ): Promise<Uint8Array>;
  export default resize;
}
