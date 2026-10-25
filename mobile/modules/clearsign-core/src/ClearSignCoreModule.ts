import { NativeModule, requireOptionalNativeModule } from 'expo';

declare class ClearSignCoreModule extends NativeModule<{}> {
  apiVersion: number;
  reviewSafeJson(jsonText: string, chainId: string, version: number): Promise<string>;
}

export default requireOptionalNativeModule<ClearSignCoreModule>('ClearSignCore');
