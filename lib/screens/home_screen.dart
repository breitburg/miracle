import 'package:flutter/cupertino.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:gap/gap.dart';
import 'package:langchain/langchain.dart';
import 'package:langchain_openai/langchain_openai.dart';
import 'package:macos_ui/macos_ui.dart';

class HomeScreen extends StatefulWidget {
  static const String routeName = '/';

  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final chat = ChatOpenAI(
    baseUrl: 'https://api.studio.nebius.com/v1',
    apiKey: String.fromEnvironment('NEBIUS_API_KEY'),
    defaultOptions: ChatOpenAIOptions(model: 'microsoft/phi-4'),
  );

  final history = <ChatMessage>[];
  final _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return MacosWindow(
      sidebar: Sidebar(
        top: MacosSearchField(
          placeholder: 'Search',
          onResultSelected: (result) {},
        ),

        builder: (context, scrollController) {
          return SidebarItems(
            currentIndex: 1,
            scrollController: scrollController,
            onChanged: (i) {},
            items: const [
              SidebarItem(label: Text('Projects'), section: true),
              SidebarItem(
                label: Row(
                  children: [
                    Icon(
                      CupertinoIcons.add,
                      size: 16,
                      color: MacosColors.systemGrayColor,
                    ),
                    Gap(10),
                    Text(
                      'Create new project',
                      style: TextStyle(color: MacosColors.systemGrayColor),
                    ),
                  ],
                ),
              ),
              SidebarItem(label: Text('Today'), section: true),
              SidebarItem(label: Text('Why is the sky blue')),
              SidebarItem(label: Text('Yesterday'), section: true),
              SidebarItem(
                label: Text('The difference between a cat and a dog'),
              ),
              SidebarItem(label: Text('Theory of relativity explained')),
              SidebarItem(label: Text('3 days ago'), section: true),
              SidebarItem(label: Text('How to grow tomatoes indoors')),
              SidebarItem(label: Text('Python vs. JavaScript')),
              SidebarItem(label: Text('2 weeks ago'), section: true),
              SidebarItem(label: Text('Recipe for banana bread')),
              SidebarItem(label: Text('Climate change effects')),
              SidebarItem(label: Text('How to deploy a React website')),
              SidebarItem(label: Text('1 month ago'), section: true),
              SidebarItem(label: Text('macOS shortcuts guide')),
              SidebarItem(label: Text('How to learn Flutter')),
              SidebarItem(label: Text('Deep learning explained')),
            ],
          );
        },
        minWidth: 200,
      ),
      child: Builder(
        builder: (context) {
          return MacosScaffold(
            toolBar: ToolBar(
              padding: const EdgeInsets.only(left: 15, right: 10),
              leading: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  MacosIconButton(
                    icon: MacosIcon(
                      CupertinoIcons.sidebar_left,
                      color: MacosTheme.brightnessOf(context).resolve(
                        const Color.fromRGBO(0, 0, 0, 0.5),
                        const Color.fromRGBO(255, 255, 255, 0.5),
                      ),
                      size: 20.0,
                    ),
                    boxConstraints: BoxConstraints(),
                    onPressed: () {
                      MacosWindowScope.of(context).toggleSidebar();
                    },
                  ),
                  const Gap(5),
                  MacosIconButton(
                    icon: MacosIcon(
                      CupertinoIcons.plus_bubble,
                      color: MacosTheme.brightnessOf(context).resolve(
                        const Color.fromRGBO(0, 0, 0, 0.5),
                        const Color.fromRGBO(255, 255, 255, 0.5),
                      ),
                      size: 20.0,
                    ),
                    boxConstraints: BoxConstraints(),
                    onPressed: () {
                      setState(() {
                        history.clear();
                        _controller.clear();
                      });
                    },
                  ),
                  const Gap(15),
                  Text(
                    'Phi-4',
                    style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500),
                  ),
                  const Gap(5),
                  MacosIcon(
                    CupertinoIcons.chevron_right,
                    size: 15,
                    color: MacosTheme.brightnessOf(context).resolve(
                      const Color.fromRGBO(0, 0, 0, 0.3),
                      const Color.fromRGBO(255, 255, 255, 0.3),
                    ),
                  ),
                ],
              ),
              actions: [
                ToolBarIconButton(
                  label: 'Share',
                  icon: MacosIcon(CupertinoIcons.share),
                  showLabel: false,
                  onPressed: () {},
                ),
                ToolBarIconButton(
                  label: 'Open in Companion Chat',
                  icon: MacosIcon(CupertinoIcons.square_on_square),
                  showLabel: false,
                  onPressed: () {},
                ),
              ],
            ),
            children: [
              ContentArea(
                builder: (
                  BuildContext context,
                  ScrollController scrollController,
                ) {
                  return Column(
                    children: [
                      Expanded(
                        child: ListView.separated(
                          reverse: true,
                          padding: const EdgeInsets.all(30),
                          itemBuilder: (context, index) {
                            final message = history.reversed.elementAt(index);
                            final out = message is HumanChatMessage;

                            if (out) {
                              return Text(
                                message.contentAsString,
                                style: MacosTheme.of(
                                  context,
                                ).typography.body.copyWith(
                                  fontSize: 14,
                                  color: MacosColors.systemGrayColor,
                                ),
                                textAlign: TextAlign.end,
                              );
                            }

                            return MarkdownBody(
                              data: message.contentAsString,
                              styleSheet: MarkdownStyleSheet(
                                p: MacosTheme.of(context).typography.body
                                    .copyWith(fontSize: 14, height: 1.6),
                                blockquote:
                                    MacosTheme.of(context).typography.body,
                                h1: MacosTheme.of(context).typography.title1,
                                h2: MacosTheme.of(context).typography.title2,
                                h3: MacosTheme.of(context).typography.title3,
                              ),
                            );
                          },
                          separatorBuilder: (context, index) => const Gap(30),
                          itemCount: history.length,
                        ),
                      ),
                      Container(
                        margin: const EdgeInsets.only(
                          bottom: 15,
                          left: 15,
                          right: 15,
                        ),
                        padding: const EdgeInsets.only(
                          left: 15,
                          right: 10,
                          bottom: 10,
                          top: 15,
                        ),
                        decoration: BoxDecoration(
                          border: Border.all(
                            color: MacosTheme.brightnessOf(context).resolve(
                              const Color.fromRGBO(0, 0, 0, 0.1),
                              const Color.fromRGBO(255, 255, 255, 0.1),
                            ),
                          ),
                          color: MacosTheme.brightnessOf(context).resolve(
                            const Color(0xFFEEEEEE),
                            const Color(0xFF3C3C3C),
                          ),
                          borderRadius: BorderRadius.circular(15),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            MacosTextField.borderless(
                              controller: _controller,
                              padding: EdgeInsets.zero,
                              style: TextStyle(fontSize: 14),
                              placeholder: 'Ask anything',
                              placeholderStyle: TextStyle(
                                color: MacosTheme.brightnessOf(context).resolve(
                                  Color.fromRGBO(0, 0, 0, 0.5),
                                  Color.fromRGBO(255, 255, 255, 0.5),
                                ),
                              ),
                              onSubmitted: (value) async {
                                if (value.isEmpty) {
                                  return;
                                }

                                final message = HumanChatMessage(
                                  content: ChatMessageContent.text(value),
                                );

                                _controller.clear();

                                setState(() {
                                  history.add(message);
                                });

                                final prompt = ChatPromptTemplate.fromPromptMessages([
                                  SystemChatMessagePromptTemplate.fromTemplate(
                                    'You are a helpful assistant who responds in a concise and informative manner. You use markdown formatting to enhance your responses, without using h1, h2, etc.',
                                  ),
                                  MessagesPlaceholder(
                                    variableName: 'chat_history',
                                  ),
                                ]);

                                final chain = prompt.pipe(chat);

                                final response = chain.stream({
                                  'chat_history': history,
                                });

                                history.add(AIChatMessage(content: ''));

                                await for (final part in response) {
                                  history.last = AIChatMessage(
                                    content:
                                        history.last.contentAsString +
                                        part.outputAsString,
                                  );
                                  setState(() {});
                                }
                              },
                            ),
                            const Gap(15),
                            Row(
                              children: [
                                MacosIcon(
                                  CupertinoIcons.plus,
                                  size: 20,
                                  color: MacosTheme.brightnessOf(
                                    context,
                                  ).resolve(
                                    const Color.fromRGBO(0, 0, 0, 0.5),
                                    const Color.fromRGBO(255, 255, 255, 0.5),
                                  ),
                                ),
                                const Gap(15),
                                MacosIcon(
                                  CupertinoIcons.globe,
                                  size: 20,
                                  color: MacosTheme.brightnessOf(
                                    context,
                                  ).resolve(
                                    const Color.fromRGBO(0, 0, 0, 0.5),
                                    const Color.fromRGBO(255, 255, 255, 0.5),
                                  ),
                                ),
                                const Gap(15),
                                MacosIcon(
                                  CupertinoIcons.cursor_rays,
                                  size: 20,
                                  color: MacosTheme.brightnessOf(
                                    context,
                                  ).resolve(
                                    const Color.fromRGBO(0, 0, 0, 0.5),
                                    const Color.fromRGBO(255, 255, 255, 0.5),
                                  ),
                                ),
                                const Spacer(),
                                MacosIcon(
                                  CupertinoIcons.mic,
                                  size: 20,
                                  color: MacosTheme.brightnessOf(
                                    context,
                                  ).resolve(
                                    const Color.fromRGBO(0, 0, 0, 0.5),
                                    const Color.fromRGBO(255, 255, 255, 0.5),
                                  ),
                                ),
                                const Gap(15),
                                Container(
                                  alignment: Alignment.center,
                                  constraints: BoxConstraints.tight(
                                    Size.square(32),
                                  ),
                                  decoration: BoxDecoration(
                                    color: MacosTheme.brightnessOf(
                                      context,
                                    ).resolve(
                                      MacosColors.black,
                                      MacosColors.white,
                                    ),
                                    shape: BoxShape.circle,
                                  ),
                                  child: Icon(
                                    CupertinoIcons.arrow_up,
                                    size: 20,
                                    color: MacosTheme.brightnessOf(
                                      context,
                                    ).resolve(
                                      MacosColors.white,
                                      MacosColors.black,
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          ],
                        ),
                      ),
                    ],
                  );
                },
              ),
            ],
          );
        },
      ),
    );
  }
}
