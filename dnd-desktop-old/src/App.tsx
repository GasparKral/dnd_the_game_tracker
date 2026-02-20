import { exit } from "@tauri-apps/plugin-process";
import { useNavigate, type NavigateFunction } from "react-router";

const App = () => {
	const navigate = useNavigate();
	return (
		<main className="flex flex-col items-center min-h-screen min-w-screen justify-center">
			<h1 className="text-center">Dungeons & Dragons</h1>
			<h2 className="mb-12">The Game Tracker</h2>

			<button className="cursor-pointer" onClick={() => createNewCampain(navigate)}>Nueva Campaña</button>
			<button className="cursor-pointer" onClick={() => loadCampain(navigate)}>Cargar Campaña</button>
			<button className="cursor-pointer" onClick={() => navigate("/options")}>Opciones</button>
			<button className="cursor-pointer" onClick={() => closeApp}>Salir</button>
		</main>
	)
}
export default App;

async function createNewCampain(navigate: NavigateFunction) { navigate("campain_creation") }

async function loadCampain(navigate: NavigateFunction) { navigate("campain") }

const closeApp = async () => {
	await exit(1);
};
