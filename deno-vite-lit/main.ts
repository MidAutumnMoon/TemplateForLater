import { LitElement, css, html } from "lit"
import { customElement, property } from "lit/decorators.js"

@customElement( "app-shell" )
export class AppShell extends LitElement {
    override render() {
        return html`
            <div>
                <h1>DEMO</h1>
                <self-changer></self-changer>
            </div>
        `
    }

    static override styles = css`
        h1 {
            color: blue;
        }
    `;
}

@customElement( "self-changer" )
export class SelfChanger extends LitElement {
    // `accessor` is important
    @property()
    accessor message = "Initial message"

    override render() {
        return html`
            <p>${this.message}</p>
        `
    }

    static override styles = css`
        p {
            font-family: monospace;
        }
    `

    override connectedCallback() {
        super.connectedCallback()
        setTimeout( () => {
            this.message = "Changed message"
        }, 1000 )
    }
}

declare global {
    interface HTMLElementTagNameMap {
        "app-shell": AppShell,
        "self-changer": SelfChanger,
    }
}
