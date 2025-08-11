<template>
  <div>
    <!-- Hero 区域 -->
    <section
      class="modern-hero"
      style="padding: 2rem 0 1.5rem 0; background: linear-gradient(135deg, #10b981 0%, #059669 100%); position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 30% 60%, rgba(255, 255, 255, 0.1) 0%, transparent 50%); opacity: 0.8;" />

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="text-align: center; color: white;">
          <!-- 现代化图标 -->
          <div style="margin: 0 auto 1rem; width: 2.5rem; height: 2.5rem; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: rgba(255, 255, 255, 0.2); backdrop-filter: blur(10px);">
            <UIcon
              name="i-heroicons-shield-check"
              style="width: 1.25rem; height: 1.25rem; color: white;"
            />
          </div>

          <!-- 现代化标题 -->
          <h1
            style="font-size: 1.875rem; font-weight: 700; line-height: 1.1; margin-bottom: 0.75rem;"
            class="md:text-2xl"
          >
            隐私政策
          </h1>

          <!-- 副标题 -->
          <p
            style="font-size: 1rem; margin: 0 auto; line-height: 1.5; opacity: 0.9;"
            class="max-w-2xl"
          >
            我们重视您的隐私，了解我们如何收集、使用和保护您的个人信息
          </p>
        </div>
      </div>
    </section>

    <!-- 面包屑导航 -->
    <nav style="background: rgba(248, 250, 252, 0.8); border-bottom: 1px solid rgba(226, 232, 240, 0.6); backdrop-filter: blur(10px);">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 0; font-size: 0.875rem;">
          <NuxtLink
            to="/"
            style="display: flex; align-items: center; gap: 0.25rem; color: #64748b; text-decoration: none; transition: all 0.2s ease;"
            class="hover:text-blue-600"
          >
            <UIcon
              name="i-heroicons-home"
              class="w-4 h-4"
            />
            首页
          </NuxtLink>
          <UIcon
            name="i-heroicons-chevron-right"
            class="w-4 h-4 text-slate-400"
          />
          <span style="color: #3b82f6; font-weight: 500;">隐私政策</span>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <section
      style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #ecfdf5 100%); padding: 3rem 0; margin: 0; position: relative;"
    >
      <!-- 背景装饰 -->
      <div style="position: absolute; inset: 0; background-image: radial-gradient(circle at 1px 1px, rgba(16, 185, 129, 0.1) 1px, transparent 0); background-size: 60px 60px; opacity: 0.5;" />

      <div class="relative mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
        <!-- 更新信息 -->
        <div
          class="modern-content-section mb-8"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
              <div
                class="modern-gradient-icon"
                style="background: linear-gradient(45deg, #10b981, #059669); width: 2.5rem; height: 2.5rem;"
              >
                <UIcon
                  name="i-heroicons-information-circle"
                  class="h-5 w-5 text-white"
                />
              </div>
              <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                政策信息
              </h2>
            </div>
          </div>

          <div class="grid md:grid-cols-2 gap-4 text-sm text-slate-600 dark:text-slate-400">
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-calendar"
                class="w-4 h-4"
              />
              <span>生效日期：2024年1月1日</span>
            </div>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-heroicons-arrow-path"
                class="w-4 h-4"
              />
              <span>最后更新：2024年1月1日</span>
            </div>
          </div>
        </div>

        <!-- 隐私承诺 -->
        <div
          class="modern-content-section mb-8"
          style="background: linear-gradient(45deg, #10b981, #059669); border-radius: 1rem; padding: 2rem; color: white; box-shadow: 0 4px 6px rgba(16, 185, 129, 0.3);"
        >
          <div class="text-center">
            <UIcon
              name="i-heroicons-shield-check"
              class="w-12 h-12 mx-auto mb-4 text-white/90"
            />
            <h2 class="text-xl font-bold mb-3">
              我们的隐私承诺
            </h2>
            <p class="text-white/90 leading-relaxed max-w-2xl mx-auto">
              ZincBloom 承诺保护您的隐私权。我们只收集必要的信息来提供服务，
              绝不会将您的个人信息出售给第三方，并采用业界标准的安全措施保护您的数据。
            </p>
          </div>
        </div>

        <!-- 政策内容 -->
        <div
          class="modern-content-section"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 3rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <div class="prose prose-slate max-w-none dark:prose-invert">
            <!-- 1. 信息收集 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-blue-500 text-white text-sm flex items-center justify-center font-medium">1</span>
                我们收集的信息
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <h3 class="font-semibold text-slate-900 dark:text-white">
                  您主动提供的信息
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li><strong>账户信息</strong>：用户名、邮箱地址、密码（加密存储）</li>
                  <li><strong>个人资料</strong>：头像、个人简介、网站链接等</li>
                  <li><strong>内容创作</strong>：您发布的文章、评论、个人动态</li>
                  <li><strong>通信记录</strong>：与我们客服的邮件往来</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white mt-6">
                  自动收集的信息
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li><strong>使用数据</strong>：访问页面、停留时间、点击行为</li>
                  <li><strong>设备信息</strong>：IP 地址、浏览器类型、操作系统</li>
                  <li><strong>技术数据</strong>：Cookie、会话标识、访问日志</li>
                </ul>
              </div>
            </section>

            <!-- 2. 信息使用 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white text-sm flex items-center justify-center font-medium">2</span>
                信息使用方式
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>我们使用收集的信息用于：</p>
                <ul class="list-disc ml-6 space-y-2">
                  <li><strong>提供服务</strong>：账户管理、内容发布、个性化体验</li>
                  <li><strong>改进产品</strong>：分析使用模式、优化功能、修复问题</li>
                  <li><strong>安全保护</strong>：防止欺诈、恶意攻击和滥用行为</li>
                  <li><strong>客户支持</strong>：响应咨询、解决技术问题</li>
                  <li><strong>法律合规</strong>：遵守相关法律法规要求</li>
                  <li><strong>通信联系</strong>：发送重要通知、安全提醒（可选择退订）</li>
                </ul>

                <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4 mt-4">
                  <div class="flex items-start gap-2">
                    <UIcon
                      name="i-heroicons-information-circle"
                      class="w-5 h-5 text-blue-600 mt-0.5"
                    />
                    <p class="text-blue-700 dark:text-blue-200 text-sm">
                      我们绝不会将您的个人信息用于营销目的，也不会向第三方出售您的数据。
                    </p>
                  </div>
                </div>
              </div>
            </section>

            <!-- 3. 信息共享 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-orange-500 text-white text-sm flex items-center justify-center font-medium">3</span>
                信息共享与披露
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>我们不会出售、出租或交易您的个人信息。仅在以下情况下可能共享信息：</p>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  经您同意的共享
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>您选择公开的个人资料信息（用户名、头像、简介等）</li>
                  <li>您发布的公开内容（文章、评论等）</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  服务提供商
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>云服务提供商（用于数据存储和处理）</li>
                  <li>CDN 服务商（用于内容分发加速）</li>
                  <li>邮件服务商（用于发送系统邮件）</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  法律要求
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>遵守法律法规、政府命令或司法程序</li>
                  <li>保护我们的权利、财产或安全</li>
                  <li>调查潜在的违规行为</li>
                </ul>
              </div>
            </section>

            <!-- 4. 数据安全 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-purple-500 text-white text-sm flex items-center justify-center font-medium">4</span>
                数据安全保护
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>我们采用多层安全措施保护您的信息：</p>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  技术安全
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li><strong>加密存储</strong>：敏感数据使用行业标准加密</li>
                  <li><strong>HTTPS 传输</strong>：所有数据传输使用 SSL/TLS 加密</li>
                  <li><strong>访问控制</strong>：严格的权限管理和身份验证</li>
                  <li><strong>安全审计</strong>：定期进行安全检查和漏洞扫描</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  管理安全
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>员工签署保密协议并接受安全培训</li>
                  <li>最小权限原则，仅必要人员可访问数据</li>
                  <li>定期备份数据以防数据丢失</li>
                </ul>

                <div class="bg-red-50 dark:bg-red-900/20 rounded-lg p-4 mt-4">
                  <div class="flex items-start gap-2">
                    <UIcon
                      name="i-heroicons-exclamation-triangle"
                      class="w-5 h-5 text-red-600 mt-0.5"
                    />
                    <p class="text-red-700 dark:text-red-200 text-sm">
                      尽管我们采取了严格的安全措施，但无法保证数据传输和存储的绝对安全。
                      请您也注意保护好自己的账户密码。
                    </p>
                  </div>
                </div>
              </div>
            </section>

            <!-- 5. Cookie 使用 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-indigo-500 text-white text-sm flex items-center justify-center font-medium">5</span>
                Cookie 和类似技术
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>我们使用 Cookie 和类似技术来改善用户体验：</p>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  必需 Cookie
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>维持登录状态和会话安全</li>
                  <li>记住用户偏好设置</li>
                  <li>防止安全威胁</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  功能 Cookie
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>记住用户的界面语言、主题设置</li>
                  <li>自动填充表单信息</li>
                  <li>提供个性化内容推荐</li>
                </ul>

                <h3 class="font-semibold text-slate-900 dark:text-white">
                  分析 Cookie
                </h3>
                <ul class="list-disc ml-6 space-y-2">
                  <li>分析网站使用情况和性能</li>
                  <li>了解用户行为模式</li>
                  <li>改进产品功能和用户体验</li>
                </ul>

                <p class="mt-4">
                  您可以通过浏览器设置控制 Cookie，但禁用某些 Cookie 可能影响网站功能。
                </p>
              </div>
            </section>

            <!-- 6. 用户权利 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-pink-500 text-white text-sm flex items-center justify-center font-medium">6</span>
                您的权利和选择
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>您对自己的个人信息享有以下权利：</p>

                <div class="grid md:grid-cols-2 gap-6">
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      访问权
                    </h3>
                    <p class="text-sm">
                      查看我们收集的关于您的个人信息
                    </p>
                  </div>
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      更正权
                    </h3>
                    <p class="text-sm">
                      更新或修正不准确的个人信息
                    </p>
                  </div>
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      删除权
                    </h3>
                    <p class="text-sm">
                      要求删除您的个人信息（在法律允许的范围内）
                    </p>
                  </div>
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      导出权
                    </h3>
                    <p class="text-sm">
                      以常见格式导出您的个人数据
                    </p>
                  </div>
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      限制处理权
                    </h3>
                    <p class="text-sm">
                      在某些情况下限制我们处理您的信息
                    </p>
                  </div>
                  <div>
                    <h3 class="font-semibold text-slate-900 dark:text-white mb-2">
                      反对权
                    </h3>
                    <p class="text-sm">
                      反对我们基于合法利益处理您的信息
                    </p>
                  </div>
                </div>

                <div class="bg-green-50 dark:bg-green-900/20 rounded-lg p-4 mt-6">
                  <div class="flex items-start gap-2">
                    <UIcon
                      name="i-heroicons-cog-6-tooth"
                      class="w-5 h-5 text-green-600 mt-0.5"
                    />
                    <div>
                      <p class="text-green-700 dark:text-green-200 text-sm font-medium mb-1">
                        如何行使权利
                      </p>
                      <p class="text-green-700 dark:text-green-200 text-sm">
                        您可以在账户设置中管理大部分信息，或联系我们行使其他权利。
                        我们会在 30 天内响应您的请求。
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <!-- 7. 数据保留 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-yellow-500 text-white text-sm flex items-center justify-center font-medium">7</span>
                数据保留期限
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>我们仅在必要期限内保留您的个人信息：</p>

                <ul class="list-disc ml-6 space-y-2">
                  <li><strong>账户信息</strong>：账户存续期间，删除账户后30天内完全清除</li>
                  <li><strong>内容数据</strong>：您主动删除后即时清除，或账户删除时一并清除</li>
                  <li><strong>访问日志</strong>：保留12个月用于安全分析，随后自动删除</li>
                  <li><strong>客服记录</strong>：保留2年用于服务改进，随后删除</li>
                  <li><strong>法律要求</strong>：某些数据可能因法律要求需保留更长时间</li>
                </ul>

                <p class="mt-4">
                  如果您长期不使用账户（超过3年），我们会提前通知您并可能删除非活跃账户。
                </p>
              </div>
            </section>

            <!-- 8. 儿童隐私 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-teal-500 text-white text-sm flex items-center justify-center font-medium">8</span>
                儿童隐私保护
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  我们非常重视儿童的隐私保护。我们的服务面向13岁以上的用户。
                  如果我们发现收集了13岁以下儿童的个人信息，会立即删除相关信息。
                </p>
                <p>
                  如果您是家长并发现您的孩子向我们提供了个人信息，请联系我们，
                  我们会采取措施删除相关信息并终止该账户。
                </p>
              </div>
            </section>

            <!-- 9. 政策变更 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-cyan-500 text-white text-sm flex items-center justify-center font-medium">9</span>
                隐私政策变更
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  我们可能会不时更新本隐私政策。重大变更时，我们会：
                </p>
                <ul class="list-disc ml-6 space-y-2">
                  <li>在网站显著位置发布变更通知</li>
                  <li>通过邮件通知注册用户（如涉及重要权益）</li>
                  <li>提供至少30天的过渡期</li>
                  <li>更新页面顶部的"最后更新"日期</li>
                </ul>
                <p>
                  继续使用我们的服务即表示您接受变更后的政策。
                  如果您不同意变更，可以停止使用服务或删除账户。
                </p>
              </div>
            </section>

            <!-- 10. 联系我们 -->
            <section class="mb-8">
              <h2 class="text-xl font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-gray-500 text-white text-sm flex items-center justify-center font-medium">10</span>
                联系我们
              </h2>
              <div class="text-slate-700 dark:text-slate-300 leading-relaxed space-y-4">
                <p>
                  如果您对本隐私政策或数据处理有任何疑问，请通过以下方式联系我们：
                </p>

                <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-6 space-y-4">
                  <div class="flex items-center gap-3">
                    <UIcon
                      name="i-heroicons-envelope"
                      class="w-5 h-5 text-slate-600"
                    />
                    <div>
                      <span class="font-medium">隐私事务邮箱：</span>
                      <a
                        href="mailto:privacy@zincbloom.com"
                        class="text-blue-600 hover:text-blue-700 ml-1"
                      >privacy@zincbloom.com</a>
                    </div>
                  </div>

                  <div class="flex items-center gap-3">
                    <UIcon
                      name="i-heroicons-identification"
                      class="w-5 h-5 text-slate-600"
                    />
                    <div>
                      <span class="font-medium">数据保护负责人：</span>
                      <span class="ml-1">ZincBloom 隐私团队</span>
                    </div>
                  </div>

                  <div class="flex items-start gap-3">
                    <UIcon
                      name="i-heroicons-clock"
                      class="w-5 h-5 text-slate-600 mt-0.5"
                    />
                    <div>
                      <span class="font-medium">响应时间：</span>
                      <span class="ml-1">我们承诺在收到您的隐私相关询问后30天内给予回复</span>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <!-- 生效声明 -->
            <div class="mt-8 pt-8 border-t border-slate-200 dark:border-slate-700">
              <div class="bg-green-50 dark:bg-green-900/20 rounded-lg p-6">
                <div class="flex items-start gap-3">
                  <UIcon
                    name="i-heroicons-shield-check"
                    class="w-5 h-5 text-green-600 mt-0.5"
                  />
                  <div>
                    <h3 class="font-medium text-green-900 dark:text-green-100 mb-2">
                      政策生效
                    </h3>
                    <p class="text-green-700 dark:text-green-200 text-sm leading-relaxed">
                      本隐私政策自2024年1月1日起生效。通过使用 ZincBloom 的服务，
                      您确认已阅读、理解并同意本隐私政策的所有条款。
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 相关链接 -->
        <div
          class="modern-content-section mt-8 text-center"
          style="background: rgba(255, 255, 255, 0.9); border-radius: 1rem; padding: 2rem; border: 1px solid #e2e8f0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);"
        >
          <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">
            相关政策
          </h3>

          <div class="flex flex-wrap items-center justify-center gap-4">
            <NuxtLink
              to="/terms"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-document-text"
                class="w-4 h-4"
              />
              服务条款
            </NuxtLink>

            <NuxtLink
              to="/help"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-question-mark-circle"
                class="w-4 h-4"
              />
              帮助中心
            </NuxtLink>

            <NuxtLink
              to="/about"
              class="inline-flex items-center gap-2 px-4 py-2 text-slate-600 hover:text-slate-900 border border-slate-200 hover:border-slate-300 rounded-lg transition-colors"
            >
              <UIcon
                name="i-heroicons-information-circle"
                class="w-4 h-4"
              />
              关于我们
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// SEO 配置
useHead({
  title: "隐私政策 - ZincBloom",
  meta: [
    { name: "description", content: "ZincBloom 隐私政策，了解我们如何收集、使用和保护您的个人信息。我们承诺保护您的隐私权，绝不将个人信息出售给第三方。" },
    { property: "og:title", content: "隐私政策 - ZincBloom" },
    { property: "og:description", content: "了解 ZincBloom 如何收集、使用和保护您的个人信息" },
    { property: "og:type", content: "website" },
  ],
});
</script>

<style scoped>
.modern-gradient-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.prose h2 {
  scroll-margin-top: 2rem;
}

.prose ul {
  margin-top: 0.5rem;
  margin-bottom: 1rem;
}

.prose li {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.prose h3 {
  margin-top: 1.5rem;
  margin-bottom: 0.5rem;
}
</style>
