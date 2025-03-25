// DB::HttpHandler::handleRequest                   -- HttpHandler.cpp:749
// DB::HttpHandler::processQuery                    -- HttpHandler.cpp:576
// DB::executeQuery                                 -- executeQuery.cpp:1891
// DB::CompletedPipelineExecutor::execute           -- CompletedPipelineExecutor.cpp:105
// DB::PipelineExecutor::execute                    -- PipelineExecutor.cpp:128
// DB::PipelineExecutor::executeImpl                -- PipelineExecutor.cpp:454
// DB::PipelineExecutor::executeSingleThread        -- PipelineExecutor.cpp:262
// DB::PipelineExecutor::executeStepImpl            -- PipelineExecutor.cpp:296
// DB::ExecutionThreadContext::executeTask          -- ExecutionThreadContext.cpp:102
// DB::executeJob                                   -- ExecutionThreadContext.cpp:53
// DB::ISource::work                                -- ISource.cpp:108
// DB::MergeTreeSource::tryGenerate                 -- MergeTreeSource.cpp:229
// DB::MergeTreeSelectProcessor::read               -- MergeTreeSelectProcessor.cpp:188
// DB::MergeTreeInOrderSelectAlgorithm::getNewTask  -- MergeTreeSelectAlgorithms.cpp:13
// DB::MergeTreeReadPoolInOrder::getTask            -- MergeTreeReadPoolInOrder.cpp:46


// JoinTreeQueryPlan buildQueryPlanForTableExpression(
// QueryTreeNodePtr table_expression,

// void ReadFromMergeTree::initializePipeline(
// QueryPipelineBuilder & pipeline, const BuildQueryPipelineSettings &)

// void MergeTreeReadPoolBase::fillPerPartInfos(const Settings & settings)

// MergeTreeRangeReader::ReadResult MergeTreeRangeReader::startReadingChain(
// size_t max_rows, MarkRanges & ranges)
//
//
// void ISerialization::deserializeBinaryBulkWithMultipleStreams(

// Merge Tree Data Part
// MergeTree::IMergeTreeDataPart

// A column is: DB::NameAndTypePair
//
// Column data types:
// DB::IDataType
//   DB::DataTypeString
