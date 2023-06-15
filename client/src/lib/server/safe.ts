export type SafeReturn<T> =
    | {
          data: T;
          success: true;
      }
    | {
          success: false;
          error: string;
      };

export async function safe<T>(
    promise: Promise<T> | T,
    err?: string,
): Promise<SafeReturn<T>> {
    try {
        const data = await promise;
        return { data, success: true };
    } catch (e) {
        console.error(e);
        if (err !== undefined) {
            return { error: err, success: false };
        }
        if (e instanceof Error) {
            return { error: e.message, success: false };
        }
        return { error: "Unknown error", success: false };
    }
}
