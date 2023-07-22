export default async function apiFetcher<T>(
    url: string,
    config: RequestInit = {}
): Promise<T> {
    const response = await fetch(url, config)

    const data = await response.json();

    let status = response.status

    if (status < 200 || status > 299) {
        throw new Error(data || status, { cause: data || "failed request with no body" })
    }

    return data as T
}