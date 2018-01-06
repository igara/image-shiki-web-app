// Components
import IndexComponent from '../components/index_component';
import AnalyzeImageComponent from '../components/analyze_image_component';

// Stores
import HeaderStore from '../stores/common/header_store';
import InputFileStore from '../stores/index/input_file_store';

const routes = {
	'/': new IndexComponent({
		HeaderStore,
		InputFileStore,
	}),
	'/analyzeimage': new AnalyzeImageComponent({
		HeaderStore,
	}),
}

export default routes;
