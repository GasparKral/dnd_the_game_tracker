export interface SiteInfoCardProps {
	isOpen: boolean;
	onClose: () => void;
	siteInfo: {
		image?: string;
		title: string;
		otherNames?: string[];
		description: string;
		relatedMissions?: Array<{
			id: string;
			name: string;
			link: string;
		}>;
	};
	position?: { x: number; y: number }; // Posición opcional para mostrar cerca del click
}

const SiteInfoCard = ({
	isOpen,
	onClose,
	siteInfo,
	position
}: SiteInfoCardProps) => {
	if (!isOpen) return null;

	return (
		<>
			{/* Overlay oscuro */}
			<div
				className="fixed inset-0 bg-black bg-opacity-50 transition-opacity z-40"
				onClick={onClose}
			/>

			{/* Card */}
			<div
				className="fixed z-50 bg-white rounded-lg shadow-xl max-w-md w-full max-h-[80vh] overflow-y-auto"
				style={{
					left: position ? `${position.x}px` : '50%',
					top: position ? `${position.y}px` : '50%',
					transform: position ? 'translate(-50%, -50%)' : 'translate(-50%, -50%)',
				}}
			>
				{/* Cabecera con image de fondo si existe */}
				{siteInfo.image && (
					<div className="h-48 overflow-hidden rounded-t-lg">
						<img
							src={siteInfo.image}
							alt={siteInfo.title}
							className="w-full h-full object-cover"
						/>
					</div>
				)}

				{/* Contenido */}
				<div className="p-6">
					{/* Título */}
					<h2 className="text-2xl font-bold text-gray-900 mb-2">
						{siteInfo.title}
					</h2>

					{/* Otros names */}
					{siteInfo.otherNames && siteInfo.otherNames.length > 0 && (
						<div className="mb-4">
							<p className="text-sm text-gray-500">También conocido como:</p>
							<div className="flex flex-wrap gap-2 mt-1">
								{siteInfo.otherNames.map((name, index) => (
									<span
										key={index}
										className="px-2 py-1 bg-gray-100 text-gray-700 text-sm rounded"
									>
										{name}
									</span>
								))}
							</div>
						</div>
					)}

					{/* Descripción */}
					<div className="mb-6">
						<h3 className="text-lg font-semibold text-gray-900 mb-2">Descripción</h3>
						<p className="text-gray-700 whitespace-pre-wrap">
							{siteInfo.description}
						</p>
					</div>

					{/* Misiones relacionadas */}
					{siteInfo.relatedMissions && siteInfo.relatedMissions.length > 0 && (
						<div>
							<h3 className="text-lg font-semibold text-gray-900 mb-2">
								Misiones relacionadas
							</h3>
							<ul className="space-y-2">
								{siteInfo.relatedMissions.map((mision) => (
									<li key={mision.id}>
										<a
											href={mision.link}
											className="text-blue-600 hover:text-blue-800 hover:underline flex items-center gap-2"
											onClick={(e) => {
												e.preventDefault();
												// Aquí manejas la navegación a la misión
												console.log('Navegar a misión:', mision);
											}}
										>
											<svg
												className="w-4 h-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													strokeLinecap="round"
													strokeLinejoin="round"
													strokeWidth={2}
													d="M13 9l3 3m0 0l-3 3m3-3H8m13 0a9 9 0 11-18 0 9 9 0 0118 0z"
												/>
											</svg>
											{mision.name}
										</a>
									</li>
								))}
							</ul>
						</div>
					)}
				</div>

				{/* Botón cerrar */}
				<button
					onClick={onClose}
					className="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
				>
					<svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		</>
	);
};

export default SiteInfoCard;
