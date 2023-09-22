import { render, fireEvent, screen } from "@testing-library/svelte";
import SocketClicker from "@components/SocketClicker.svelte";

describe("SocketClicker", () => {
    // global test values here

    test("increment when clicked", async () => {
        render(SocketClicker);
        const button = screen.getByRole("button");
        expect(button).toHaveTextContent(/^clicked 0 times$/i);

        await fireEvent.click(button);
        expect(button).toHaveTextContent(/^clicked 1 time$/i);

        await fireEvent.click(button);
        expect(button).toHaveTextContent(/^clicked 2 times$/i);
    });
});
