<button id="theme-toggle"></button>
<h1>
    Em Python a yfinance, em JS ( Node ) a yahoo-finance e yahoo-finance2, e em Rust, o que você verá a seguir.
</h1>
<p>
    Se mesmo em node, a yahoo finance precisa ser executada no lado do servidor, por que não facilitar essa API na linguagem mais segura do mundo e que ainda tem uma performance excelente? É a linguagem perfeita para o back-end.
</p>
<h2>
    Como usar:
</h2>
<h2>
    Detalhes técnicos:
</h2>
<footer>
    <address>
        <h3>Referências</h3>
        <div id="rodape">
            <a href="https://pypi.org/project/yfinance/" class="gitLink"><button>
                <p>yfinance</p>
            </button></a>
            <a href="https://github.com/gadicc/node-yahoo-finance2" class="gitLink"><button>
                <p>node-yahoo-finance-2</p>
            </button></a>
            <a href="https://github.com/pilwon/node-yahoo-finance" class="gitLink"><button>
                <p>node-yahoo-finance</p>
            </button></a>
        </div>
    </address>
</footer>
<style>
    body {
        background-color: white;
        color: black;
    }
    .dark-theme {
        body {
            background-color: black;
            color: white;
        }
    }
    button {
        border-radius: 2000px;
        border: black solid 1px;
        margin: 5px;
        display: flex;
        background-color: white;
        color: black;
    }
    button:hover {
        background-color: black;
        color: white;
    }
    a {
        text-decoration: none;
        outline: none;
    }
    button p, svg p {
        margin: 10px;
    }
    #rodape, address {
        display: flex;
        align-items: center;
        justify-content: center;
    }
    address {
        flex-direction: column;
    }
</style>
<script>
    const moon = `<svg id="moon" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><g fill="none"><path fill="currentColor" d="M12 21a9 9 0 0 0 8.997-9.252a7 7 0 0 1-10.371-8.643A9 9 0 0 0 12 21" opacity=".16" /><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 21a9 9 0 0 0 8.997-9.252a7 7 0 0 1-10.371-8.643A9 9 0 0 0 12 21" /></g></svg>`;
    const sun = `<svg id="sun" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><g fill="none"><circle cx="12" cy="12" r="4" fill="currentColor" opacity=".16" /><circle cx="12" cy="12" r="4" stroke="currentColor" stroke-linejoin="round" stroke-width="2" /><path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M20 12h1M3 12h1m8 8v1m0-18v1m5.657 13.657l.707.707M5.636 5.636l.707.707m0 11.314l-.707.707M18.364 5.636l-.707.707" /></g></svg>`;
    const gitHub = `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 16 16"><path fill="currentColor" d="M8 1C4.13 1 1 4.21 1 8.18c0 3.14 1.96 5.81 4.69 6.79c.37.09.31-.17.31-.36v-1.25c-2.12.26-2.21-1.19-2.35-1.43c-.29-.5-.97-.63-.76-.87c.48-.26.98.06 1.55.93c.41.63 1.22.52 1.63.42c.09-.38.28-.71.54-.98c-2.2-.4-3.12-1.78-3.12-3.42c0-.8.25-1.53.76-2.12c-.32-.97.03-1.8.08-1.93c.91-.08 1.85.67 1.93.73c.52-.14 1.11-.22 1.77-.22s1.25.08 1.78.22c.18-.14 1.05-.78 1.9-.71c.05.12.39.94.09 1.9c.51.59.76 1.33.76 2.13c0 1.64-.92 3.02-3.13 3.42a2 2 0 0 1 .44.67c.1.25.15.52.15.79v1.81c.01.14 0 .29.23.29c2.77-.96 4.76-3.65 4.76-6.81c0-3.97-3.13-7.19-7-7.19Z"/></svg>`;
    const theme = `<button id="theme-toggle">${sun, moon}</button>`;
    function changeTheme () {
    document.addEventListener("DOMContentLoaded", function () {
        const themeToggle = document.getElementById("theme-toggle");
        const body = document.querySelector("body");
        const currentTheme = localStorage.getItem("theme");
        if (currentTheme) {
            body.classList.add(currentTheme);
        }
        themeToggle.addEventListener("click", function () {
            if (body.classList.contains("light-theme")) {
                body.classList.remove("light-theme");
                body.classList.add("dark-theme");
                localStorage.setItem("theme", "dark-theme");
            } else {
                body.classList.remove("dark-theme");
                body.classList.add("light-theme");
                localStorage.setItem("theme", "light-theme");
            }
        });
    });  
    }
</script>
