const BASE_URI = "http://localhost:8000"

interface LoginResponse {
    login: string
}
export function login(email: string, password: string): Promise<LoginResponse> {
    const query = `mutation {
        login(email: "${email}", password: "${password}")
    }`.replace(/\s\s*/g, " ")
    return fetch(`${BASE_URI}/graphql`, {
        method: "POST",
        body: JSON.stringify({
            query: query,
        }),
        headers: {
            "Content-Type": "application/json"
        }
    })
    .then(response => response.json())
    .then(success)
}

export function success(result: any) {
    if (result.errors) {
        throw { errors: result.errors }
    } else {
        return result.data
    }
}
