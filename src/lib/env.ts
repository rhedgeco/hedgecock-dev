export default {
    apiOrigin: () => {
        return `${location.origin}/api`;
    },
    wsOrigin: () => {
        let protocol = location.protocol === "https:" ? "wss:" : "ws:";
        return `${protocol}//${location.host}/api`;
    },
};
